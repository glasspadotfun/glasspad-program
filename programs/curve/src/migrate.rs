//! graduation. anyone can call it once the curve is complete: the curve's sol and
//! tokens go into a raydium cpmm pool, the lp is burned in the same instruction,
//! and the 1 sol migration fee (after raydium's pool fee and rent) is split like
//! every protocol fee: half to the team wallet, half to the burn pda.
use crate::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{Mint, TokenAccount}};
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke_signed};

/// raydium cpmm (mainnet; the `devnet` feature switches all three to raydium's devnet deployment).
#[cfg(not(feature = "devnet"))]
pub const CPMM_PROGRAM: Pubkey = anchor_lang::solana_program::pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
#[cfg(feature = "devnet")]
pub const CPMM_PROGRAM: Pubkey = anchor_lang::solana_program::pubkey!("DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb");
/// raydium's pool creation fee receiver (a wsol token account).
#[cfg(not(feature = "devnet"))]
pub const CPMM_CREATE_POOL_FEE: Pubkey = anchor_lang::solana_program::pubkey!("DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8");
#[cfg(feature = "devnet")]
pub const CPMM_CREATE_POOL_FEE: Pubkey = anchor_lang::solana_program::pubkey!("3oE58BKVt8KuYkGxx8zBojugnymWmBiyafWgMrnb6eYy");
/// raydium cpmm config index 0 (0.25% trade fee).
#[cfg(not(feature = "devnet"))]
pub const CPMM_AMM_CONFIG: Pubkey = anchor_lang::solana_program::pubkey!("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2");
#[cfg(feature = "devnet")]
pub const CPMM_AMM_CONFIG: Pubkey = anchor_lang::solana_program::pubkey!("5MxLgy9oPdTC3YgkiePHqr3EoCRD9uLVYRQS2ANAs7wy");
const CPMM_AUTH_SEED: &[u8] = b"vault_and_lp_mint_auth_seed";
const CPMM_POOL_SEED: &[u8] = b"pool";
const CPMM_LP_MINT_SEED: &[u8] = b"pool_lp_mint";
const CPMM_VAULT_SEED: &[u8] = b"pool_vault";
const CPMM_OBSERVATION_SEED: &[u8] = b"observation";
/// sha256("global:initialize")[..8]
const CPMM_INITIALIZE_DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

#[event_cpi]
#[derive(Accounts)]
pub struct Migrate<'info> {
    #[account(seeds = [b"global"], bump)]
    pub global: Box<Account<'info, Global>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut, seeds = [b"bonding-curve", mint.key().as_ref()], bump, constraint = bonding_curve.quote_mint == Pubkey::default() @ PumpError::UnsupportedQuoteMint)]
    pub bonding_curve: Box<Account<'info, crate::trade_types::BondingCurve>>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = bonding_curve, associated_token::token_program = token_program)]
    pub associated_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,
    /// anyone. pays nothing but the tx fee.
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: our pda, the pool's creator on raydium. holds the migration fee while the pool is made.
    #[account(mut, seeds = [b"pool-authority", mint.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: ata of pool_authority for the coin, made here.
    #[account(mut)]
    pub pool_authority_mint_account: UncheckedAccount<'info>,
    /// CHECK: ata of pool_authority for wsol, made here.
    #[account(mut)]
    pub pool_authority_wsol_account: UncheckedAccount<'info>,
    /// CHECK: ata of pool_authority for the lp, made by raydium, burned and closed here.
    #[account(mut)]
    pub pool_authority_lp_account: UncheckedAccount<'info>,
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub wsol_mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: the team wallet, pinned in the source.
    #[account(mut, address = crate::consts::TEAM)]
    pub team: UncheckedAccount<'info>,
    /// CHECK: the burn pda.
    #[account(mut, seeds = [crate::consts::BURN_SEED], bump)]
    pub burn_pda: UncheckedAccount<'info>,
    // raydium cpmm
    /// CHECK: pinned program id.
    #[account(address = CPMM_PROGRAM)]
    pub cpmm_program: UncheckedAccount<'info>,
    /// CHECK: pinned config.
    #[account(address = CPMM_AMM_CONFIG)]
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: raydium's vault and lp mint authority.
    #[account(seeds = [CPMM_AUTH_SEED], bump, seeds::program = cpmm_program.key())]
    pub cpmm_authority: UncheckedAccount<'info>,
    /// CHECK: token 0 and 1 are the coin and wsol in raydium's order (lower pubkey first).
    #[account(constraint = (token_0_mint.key() == mint.key() && token_1_mint.key() == wsol_mint.key()) || (token_0_mint.key() == wsol_mint.key() && token_1_mint.key() == mint.key()) @ PumpError::NotAuthorized, constraint = token_0_mint.key() < token_1_mint.key() @ PumpError::NotAuthorized)]
    pub token_0_mint: UncheckedAccount<'info>,
    /// CHECK: see token_0_mint.
    pub token_1_mint: UncheckedAccount<'info>,
    /// CHECK: raydium pool pda.
    #[account(mut, seeds = [CPMM_POOL_SEED, amm_config.key().as_ref(), token_0_mint.key().as_ref(), token_1_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub pool_state: UncheckedAccount<'info>,
    /// CHECK: raydium lp mint pda.
    #[account(mut, seeds = [CPMM_LP_MINT_SEED, pool_state.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub lp_mint: UncheckedAccount<'info>,
    /// CHECK: raydium vault pda.
    #[account(mut, seeds = [CPMM_VAULT_SEED, pool_state.key().as_ref(), token_0_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub token_0_vault: UncheckedAccount<'info>,
    /// CHECK: raydium vault pda.
    #[account(mut, seeds = [CPMM_VAULT_SEED, pool_state.key().as_ref(), token_1_mint.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub token_1_vault: UncheckedAccount<'info>,
    /// CHECK: pinned by address.
    #[account(mut, address = CPMM_CREATE_POOL_FEE)]
    pub create_pool_fee: UncheckedAccount<'info>,
    /// CHECK: raydium observation pda.
    #[account(mut, seeds = [CPMM_OBSERVATION_SEED, pool_state.key().as_ref()], bump, seeds::program = cpmm_program.key())]
    pub observation_state: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn execute<'info>(ctx: Context<'_, '_, '_, 'info, Migrate<'info>>) -> Result<()> {
    let a = &ctx.accounts;
    crate::trade_types::prepare_bonding_curve(&a.bonding_curve, &a.system_program, &a.user)?;
    require!(a.bonding_curve.complete, PumpError::BondingCurveNotComplete);
    require!(a.bonding_curve.real_quote_reserves != 0 || a.associated_bonding_curve.amount != 0, PumpError::BondingCurveComplete);
    let fee = a.global.pool_migration_fee;
    let bump = ctx.bumps.pool_authority;
    let mint_key = a.mint.key();
    let signer: &[&[&[u8]]] = &[&[b"pool-authority", mint_key.as_ref(), &[bump]]];

    // 1. the migration fee moves from the curve to pool_authority; raydium's costs come out of it.
    a.bonding_curve.sub_lamports(fee)?;
    a.pool_authority.add_lamports(fee)?;

    // 2. pool_authority's token and wsol accounts.
    create_ata(a, &a.pool_authority_mint_account, &a.mint.to_account_info(), signer)?;
    create_ata(a, &a.pool_authority_wsol_account, &a.wsol_mint.to_account_info(), signer)?;

    // 3. every token left on the curve, and every lamport of real sol, go to pool_authority.
    let base = a.associated_bonding_curve.amount;
    let curve_bump = ctx.bumps.bonding_curve;
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            a.token_program.to_account_info(),
            anchor_spl::token::Transfer { from: a.associated_bonding_curve.to_account_info(), to: a.pool_authority_mint_account.to_account_info(), authority: a.bonding_curve.to_account_info() },
            &[&[b"bonding-curve", mint_key.as_ref(), &[curve_bump]]],
        ),
        base,
    )?;
    let quote = a.bonding_curve.real_quote_reserves
        .checked_add(a.bonding_curve.post_complete_quote_in).ok_or_else(|| error!(PumpError::Overflow))?
        .checked_sub(fee).ok_or_else(|| error!(PumpError::Overflow))?;
    a.bonding_curve.sub_lamports(quote)?;
    a.pool_authority_wsol_account.add_lamports(quote)?;
    // the runtime checks the caller's balances at every cpi over the accounts it passes, so the two
    // accounts just touched ride along in one more (idempotent, no-op) ata create before the sync.
    create_ata(a, &a.pool_authority_wsol_account, &a.wsol_mint.to_account_info(), signer)?;
    anchor_spl::token::sync_native(CpiContext::new(a.token_program.to_account_info(), anchor_spl::token::SyncNative { account: a.pool_authority_wsol_account.to_account_info() }))?;

    // 4. the pool, signed by pool_authority as raydium's "creator".
    let coin_is_0 = a.token_0_mint.key() == mint_key;
    let (amount_0, amount_1) = if coin_is_0 { (base, quote) } else { (quote, base) };
    let (creator_0, creator_1) = if coin_is_0 {
        (a.pool_authority_mint_account.to_account_info(), a.pool_authority_wsol_account.to_account_info())
    } else {
        (a.pool_authority_wsol_account.to_account_info(), a.pool_authority_mint_account.to_account_info())
    };
    let mut data = CPMM_INITIALIZE_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&amount_0.to_le_bytes());
    data.extend_from_slice(&amount_1.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    let ix = Instruction {
        program_id: CPMM_PROGRAM,
        accounts: vec![
            AccountMeta::new(a.pool_authority.key(), true),
            AccountMeta::new_readonly(a.amm_config.key(), false),
            AccountMeta::new_readonly(a.cpmm_authority.key(), false),
            AccountMeta::new(a.pool_state.key(), false),
            AccountMeta::new_readonly(a.token_0_mint.key(), false),
            AccountMeta::new_readonly(a.token_1_mint.key(), false),
            AccountMeta::new(a.lp_mint.key(), false),
            AccountMeta::new(creator_0.key(), false),
            AccountMeta::new(creator_1.key(), false),
            AccountMeta::new(a.pool_authority_lp_account.key(), false),
            AccountMeta::new(a.token_0_vault.key(), false),
            AccountMeta::new(a.token_1_vault.key(), false),
            AccountMeta::new(a.create_pool_fee.key(), false),
            AccountMeta::new(a.observation_state.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new_readonly(a.associated_token_program.key(), false),
            AccountMeta::new_readonly(a.system_program.key(), false),
            AccountMeta::new_readonly(a.rent.key(), false),
        ],
        data,
    };
    invoke_signed(
        &ix,
        &[
            a.pool_authority.to_account_info(), a.amm_config.to_account_info(), a.cpmm_authority.to_account_info(), a.pool_state.to_account_info(),
            a.token_0_mint.to_account_info(), a.token_1_mint.to_account_info(), a.lp_mint.to_account_info(), creator_0, creator_1,
            a.pool_authority_lp_account.to_account_info(), a.token_0_vault.to_account_info(), a.token_1_vault.to_account_info(),
            a.create_pool_fee.to_account_info(), a.observation_state.to_account_info(), a.token_program.to_account_info(),
            a.associated_token_program.to_account_info(), a.system_program.to_account_info(), a.rent.to_account_info(), a.cpmm_program.to_account_info(),
        ],
        signer,
    )?;

    // 5. burn every lp token and close the three temp accounts back to pool_authority.
    let lp_amount = {
        let data = a.pool_authority_lp_account.try_borrow_data()?;
        anchor_spl::token::TokenAccount::try_deserialize_unchecked(&mut &data[..])?.amount
    };
    anchor_spl::token::burn(
        CpiContext::new_with_signer(a.token_program.to_account_info(), anchor_spl::token::Burn { mint: a.lp_mint.to_account_info(), from: a.pool_authority_lp_account.to_account_info(), authority: a.pool_authority.to_account_info() }, signer),
        lp_amount,
    )?;
    for acc in [&a.pool_authority_lp_account, &a.pool_authority_mint_account, &a.pool_authority_wsol_account] {
        anchor_spl::token::close_account(CpiContext::new_with_signer(
            a.token_program.to_account_info(),
            anchor_spl::token::CloseAccount { account: acc.to_account_info(), destination: a.pool_authority.to_account_info(), authority: a.pool_authority.to_account_info() },
            signer,
        ))?;
    }

    // 6. what is left of the fee: half team, half burn pda.
    let left = a.pool_authority.lamports();
    let to_burn = left / 2;
    let to_team = left - to_burn;
    crate::payments::migrate_transfer_native(a.system_program.as_ref(), &a.pool_authority, &a.burn_pda.to_account_info(), to_burn, &[&[b"pool-authority", mint_key.as_ref(), &[bump]]])?;
    crate::payments::migrate_transfer_native(a.system_program.as_ref(), &a.pool_authority, &a.team.to_account_info(), to_team, &[&[b"pool-authority", mint_key.as_ref(), &[bump]]])?;

    {
        let c: &mut crate::trade_types::BondingCurve = &mut ctx.accounts.bonding_curve;
        c.real_quote_reserves = 0; c.real_token_reserves = 0; c.virtual_quote_reserves = 0; c.virtual_token_reserves = 0;
    }
    let a = &ctx.accounts;
    let event = GraduateEvent { user: a.user.key(), mint: mint_key, token_amount: base, sol_amount: quote, migration_fee: fee, team_fee: to_team, burn_fee: to_burn, pool: a.pool_state.key(), lp_burned: lp_amount, timestamp: Clock::get()?.unix_timestamp };
    emit!(event); emit_cpi!(event);
    Ok(())
}

fn create_ata<'info>(a: &Migrate<'info>, account: &UncheckedAccount<'info>, mint: &AccountInfo<'info>, signer: &[&[&[u8]]]) -> Result<()> {
    let ix = Instruction {
        program_id: anchor_spl::associated_token::ID,
        accounts: vec![
            AccountMeta::new(a.pool_authority.key(), true),
            AccountMeta::new(account.key(), false),
            AccountMeta::new_readonly(a.pool_authority.key(), false),
            AccountMeta::new_readonly(mint.key(), false),
            AccountMeta::new_readonly(a.system_program.key(), false),
            AccountMeta::new_readonly(a.token_program.key(), false),
            AccountMeta::new(a.bonding_curve.key(), false), // rides along so the runtime sees the fee move balanced
        ],
        data: vec![1], // create idempotent
    };
    invoke_signed(&ix, &[a.pool_authority.to_account_info(), account.to_account_info(), a.pool_authority.to_account_info(), mint.clone(), a.system_program.to_account_info(), a.token_program.to_account_info(), a.bonding_curve.to_account_info()], signer)?;
    Ok(())
}

#[event]
#[derive(Clone)]
pub struct GraduateEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub token_amount: u64,
    pub sol_amount: u64,
    pub migration_fee: u64,
    pub team_fee: u64,
    pub burn_fee: u64,
    pub pool: Pubkey,
    pub lp_burned: u64,
    pub timestamp: i64,
}

// shims for modules that are dead but still compiled until change 7 removes them.
pub use crate::payments::migrate_transfer_native as transfer_native;
#[derive(Clone)] pub struct DistributionCurve { pub creator: Pubkey, pub quote_mint: Pubkey }
impl Owner for DistributionCurve { fn owner() -> Pubkey { crate::ID } }
impl AccountSerialize for DistributionCurve {}
impl AccountDeserialize for DistributionCurve {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> { Self::try_deserialize_unchecked(buf) }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        if buf.len() < 115 || buf[..8] != [23, 183, 248, 55, 96, 216, 172, 96] { return Err(anchor_lang::solana_program::program_error::ProgramError::InvalidAccountData.into()); }
        Ok(Self { creator: Pubkey::new_from_array(buf[49..81].try_into().unwrap_or_default()), quote_mint: Pubkey::new_from_array(buf[83..115].try_into().unwrap_or_default()) })
    }
}
