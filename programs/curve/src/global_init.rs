use anchor_lang::prelude::*;
use crate::{consts, Initialize};
/// runs once, from the deployer. writes the constants into `global` and keeps no authority:
/// there is no instruction left that can write to `global` after this.
pub fn execute(ctx:Context<Initialize>)->Result<()>{
 let g=&mut ctx.accounts.global;
 let burn=consts::burn_pda();
 g.initialized=true;
 g.authority=Pubkey::default();
 g.fee_recipient=consts::TEAM;
 g.fee_recipients=[consts::TEAM;7];
 g.initial_virtual_token_reserves=consts::INITIAL_VIRTUAL_TOKEN_RESERVES;
 g.initial_virtual_sol_reserves=consts::INITIAL_VIRTUAL_SOL_RESERVES;
 g.initial_virtual_quote_reserves=consts::INITIAL_VIRTUAL_SOL_RESERVES;
 g.initial_real_token_reserves=consts::INITIAL_REAL_TOKEN_RESERVES;
 g.token_total_supply=consts::TOKEN_TOTAL_SUPPLY;
 g.fee_basis_points=consts::PROTOCOL_FEE_BPS;
 g.creator_fee_basis_points=consts::CREATOR_FEE_BPS;
 g.buyback_basis_points=consts::BUYBACK_BPS;
 g.buyback_fee_recipients=[burn;8];
 g.pool_migration_fee=consts::POOL_MIGRATION_FEE;
 g.withdraw_authority=consts::TEAM;
 g.enable_migrate=true;
 g.set_creator_authority=Pubkey::default();
 g.admin_set_creator_authority=Pubkey::default();
 g.create_v2_enabled=false;
 g.whitelist_pda=Pubkey::default();
 g.reserved_fee_recipient=Pubkey::default();
 g.reserved_fee_recipients=[Pubkey::default();7];
 g.mayhem_mode_enabled=false;
 g.is_cashback_enabled=false;
 g.whitelisted_quote_mints=[Pubkey::default();1];
 g.creator_fee_configurable=false;
 g.max_configurable_creator_fee_bps=0;
 g.platform_coin=Pubkey::default();
 g.is_holder_reward_enabled=false;
 g.max_curve_depth=0;
 Ok(())
}
#[event_cpi]
#[derive(Accounts)]
pub struct UpdateGlobalAuthority<'info>{
 #[account(mut,seeds=[b"global"],bump,has_one=authority @ crate::PumpError::NotAuthorized)]
 pub global:Account<'info,crate::Global>,
 pub authority:Signer<'info>,
 /// CHECK: new authority is assigned by the current signer; zero/self are accepted by the original.
 pub new_authority:AccountInfo<'info>,
}
