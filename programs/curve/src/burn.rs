//! the burn half of the platform fee. `seal_platform_coin` names the platform coin
//! once (the only write to `global` after `initialize`, and it seals itself).
//! `burn_crank` is permissionless: it swaps the burn pda's sol for the platform
//! coin on its raydium cpmm pool, with a minimum out computed from the pool's own
//! reserves in the same transaction, and burns every token it gets.
use crate::*;
use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::Mint};
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke_signed};
use crate::migrate::{CPMM_PROGRAM, CPMM_AMM_CONFIG};

const CPMM_AUTH_SEED: &[u8] = b"vault_and_lp_mint_auth_seed";
const CPMM_POOL_SEED: &[u8] = b"pool";
const CPMM_VAULT_SEED: &[u8] = b"pool_vault";
const CPMM_OBSERVATION_SEED: &[u8] = b"observation";
/// sha256("global:swap_base_input")[..8]
const CPMM_SWAP_BASE_INPUT_DISCRIMINATOR: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
/// raydium cpmm config 0 trade fee: 0.25% of the input, in 1/1_000_000.
const CPMM_TRADE_FEE_RATE: u128 = 2_500;
/// the crank accepts at most 1% less than the reserves say (sandwich-proof: same tx).
const SLIPPAGE_BPS: u128 = 100;
/// lamports the burn pda keeps so it stays rent-exempt and the crank always has a tx to run.
const BURN_PDA_FLOOR: u64 = 10_000_000;

#[derive(Accounts)]
pub struct SealPlatformCoin<'info> {
    #[account(mut, seeds = [b"global"], bump, constraint = global.platform_coin == Pubkey::default() @ PumpError::NotAuthorized)]
    pub global: Account<'info, Global>,
    /// the team wallet, pinned in the source, signs exactly once.
    #[account(address = crate::consts::TEAM @ PumpError::NotAuthorized)]
    pub team: Signer<'info>,
    pub platform_coin: InterfaceAccount<'info, Mint>,
}

pub fn seal_platform_coin(ctx: Context<SealPlatformCoin>) -> Result<()> {
    ctx.accounts.global.platform_coin = ctx.accounts.platform_coin.key();
    emit!(PlatformCoinSealedEvent { platform_coin: ctx.accounts.platform_coin.key(), timestamp: Clock::get()?.unix_timestamp });
    Ok(())
}

#[event_cpi]
#[derive(Accounts)]
pub struct BurnCrank<'info> {
    #[account(seeds = [b"global"], bump, constraint = global.platform_coin != Pubkey::default() @ PumpError::NotAuthorized)]
    pub global: Box<Account<'info, Global>>,
    /// anyone. pays the tx fee and nothing else.
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: holds the burn half of every fee.
    #[account(mut, seeds = [crate::consts::BURN_SEED], bump)]
    pub burn_pda: UncheckedAccount<'info>,
    #[account(address = global.platform_coin)]
    pub platform_coin: Box<InterfaceAccount<'info, Mint>>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub wsol_mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: burn pda's wsol ata, made here, closed here.
    #[account(mut)]
    pub burn_wsol_account: UncheckedAccount<'info>,
    /// CHECK: burn pda's platform coin ata, made here, emptied here.
    #[account(mut)]
    pub burn_coin_account: UncheckedAccount<'info>,
    /// CHECK: pinned program id.
    #[account(address = CPMM_PROGRAM)]
    pub cpmm_program: UncheckedAccount<'info>,
    /// CHECK: pinned config.
    #[account(address = CPMM_AMM_CONFIG)]
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: raydium's vault authority.
    #[account(seeds = [CPMM_AUTH_SEED], bump, seeds::program = cpmm_program.key())]
    pub cpmm_authority: UncheckedAccount<'info>,
    /// CHECK: the platform coin and wsol in raydium's order.
    #[account(constraint = (token_0_mint.key() == platform_coin.key() && token_1_mint.key() == wsol_mint.key()) || (token_0_mint.key() == wsol_mint.key() && token_1_mint.key() == platform_coin.key()) @ PumpError::NotAuthorized, constraint = token_0_mint.key() < token_1_mint.key() @ PumpError::NotAuthorized)]
    pub token_0_mint: UncheckedAccount<'info>,
    /// CHECK: see token_0_mint.
    pub token_1_mint: UncheckedAccount<'info>,
    /// CHECK: the one pool the crank can buy from: config 0, platform coin / wsol.
    #[account(mut, seeds = [CPMM_POOL_SEED, amm_config.key().as_ref(), token_0_mint.key().as_ref(), token_1_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub pool_state: UncheckedAccount<'info>,
    /// CHECK: raydium vault pda.
    #[account(mut, seeds = [CPMM_VAULT_SEED, pool_state.key().as_ref(), token_0_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub token_0_vault: UncheckedAccount<'info>,
    /// CHECK: raydium vault pda.
    #[account(mut, seeds = [CPMM_VAULT_SEED, pool_state.key().as_ref(), token_1_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub token_1_vault: UncheckedAccount<'info>,
    /// CHECK: raydium observation pda.
    #[account(mut, seeds = [CPMM_OBSERVATION_SEED, pool_state.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub observation_state: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

fn token_amount(account: &AccountInfo) -> Result<u64> {
    let data = account.try_borrow_data()?;
    Ok(anchor_spl::token::TokenAccount::try_deserialize_unchecked(&mut &data[..])?.amount)
}

pub fn burn_crank(ctx: Context<BurnCrank>) -> Result<()> {
    let a = &ctx.accounts;
    let bump = ctx.bumps.burn_pda;
    let signer: &[&[&[u8]]] = &[&[crate::consts::BURN_SEED, &[bump]]];
    let amount_in = a.burn_pda.lamports().saturating_sub(BURN_PDA_FLOOR);
    require_gt!(amount_in, 0, PumpError::BuyZeroAmount);

    // atas for the burn pda (idempotent).
    for (acc, mint) in [(&a.burn_wsol_account, a.wsol_mint.to_account_info()), (&a.burn_coin_account, a.platform_coin.to_account_info())] {
        let ix = Instruction {
            program_id: anchor_spl::associated_token::ID,
            accounts: vec![
                AccountMeta::new(a.burn_pda.key(), true),
                AccountMeta::new(acc.key(), false),
                AccountMeta::new_readonly(a.burn_pda.key(), false),
                AccountMeta::new_readonly(mint.key(), false),
                AccountMeta::new_readonly(a.system_program.key(), false),
                AccountMeta::new_readonly(a.token_program.key(), false),
            ],
            data: vec![1],
        };
        invoke_signed(&ix, &[a.burn_pda.to_account_info(), acc.to_account_info(), a.burn_pda.to_account_info(), mint, a.system_program.to_account_info(), a.token_program.to_account_info()], signer)?;
    }
    // the ata creation above may have spent rent from the pda; recompute what is left to swap.
    let amount_in = a.burn_pda.lamports().saturating_sub(BURN_PDA_FLOOR);
    require_gt!(amount_in, 0, PumpError::BuyZeroAmount);

    // wrap.
    a.burn_pda.sub_lamports(amount_in)?;
    a.burn_wsol_account.add_lamports(amount_in)?;
    anchor_spl::token::sync_native(CpiContext::new(a.token_program.to_account_info(), anchor_spl::token::SyncNative { account: a.burn_wsol_account.to_account_info() }))?;

    // minimum out from the pool's reserves, read in this same transaction.
    let coin_is_0 = a.token_0_mint.key() == a.platform_coin.key();
    let (input_vault, output_vault) = if coin_is_0 { (&a.token_1_vault, &a.token_0_vault) } else { (&a.token_0_vault, &a.token_1_vault) };
    let reserve_in = token_amount(&input_vault.to_account_info())? as u128;
    let reserve_out = token_amount(&output_vault.to_account_info())? as u128;
    let in_after_fee = (amount_in as u128) * (1_000_000 - CPMM_TRADE_FEE_RATE) / 1_000_000;
    let expected_out = reserve_out * in_after_fee / (reserve_in + in_after_fee);
    let minimum_out = u64::try_from(expected_out * (10_000 - SLIPPAGE_BPS) / 10_000).map_err(|_| error!(PumpError::Truncation))?;
    require_gt!(minimum_out, 0, PumpError::BuyZeroAmount);

    // swap on raydium, signed by the burn pda.
    let mut data = CPMM_SWAP_BASE_INPUT_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&minimum_out.to_le_bytes());
    let ix = Instruction {
        program_id: CPMM_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(a.burn_pda.key(), true),
            AccountMeta::new_readonly(a.cpmm_authority.key(), false),
            AccountMeta::new_readonly(a.amm_config.key(), false),
            AccountMeta::new(a.pool_state.key(), false),
            AccountMeta::new(a.burn_wsol_account.key(), false),
            AccountMeta::new(a.burn_coin_account.key(), false),
            AccountMeta::new(input_vault.key(), false),
            AccountMeta::new(output_vault.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new_readonly(a.wsol_mint.key(), false),
            AccountMeta::new_readonly(a.platform_coin.key(), false),
            AccountMeta::new(a.observation_state.key(), false),
        ],
        data,
    };
    invoke_signed(
        &ix,
        &[
            a.burn_pda.to_account_info(), a.cpmm_authority.to_account_info(), a.amm_config.to_account_info(), a.pool_state.to_account_info(),
            a.burn_wsol_account.to_account_info(), a.burn_coin_account.to_account_info(), input_vault.to_account_info(), output_vault.to_account_info(),
            a.token_program.to_account_info(), a.wsol_mint.to_account_info(), a.platform_coin.to_account_info(), a.observation_state.to_account_info(),
            a.cpmm_program.to_account_info(),
        ],
        signer,
    )?;

    // burn everything the swap returned, and close the wsol account back to the pda.
    let bought = token_amount(&a.burn_coin_account.to_account_info())?;
    anchor_spl::token::burn(
        CpiContext::new_with_signer(a.token_program.to_account_info(), anchor_spl::token::Burn { mint: a.platform_coin.to_account_info(), from: a.burn_coin_account.to_account_info(), authority: a.burn_pda.to_account_info() }, signer),
        bought,
    )?;
    anchor_spl::token::close_account(CpiContext::new_with_signer(
        a.token_program.to_account_info(),
        anchor_spl::token::CloseAccount { account: a.burn_wsol_account.to_account_info(), destination: a.burn_pda.to_account_info(), authority: a.burn_pda.to_account_info() },
        signer,
    ))?;

    let event = BurnEvent { user: a.user.key(), sol_in: amount_in, coin_burned: bought, pool: a.pool_state.key(), timestamp: Clock::get()?.unix_timestamp };
    emit!(event); emit_cpi!(event);
    Ok(())
}

#[event]
#[derive(Clone)]
pub struct PlatformCoinSealedEvent { pub platform_coin: Pubkey, pub timestamp: i64 }

#[event]
#[derive(Clone)]
pub struct BurnEvent { pub user: Pubkey, pub sol_in: u64, pub coin_burned: u64, pub pool: Pubkey, pub timestamp: i64 }
