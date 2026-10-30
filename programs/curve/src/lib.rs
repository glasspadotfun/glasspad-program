pub mod consts;
pub mod burn;
pub use burn::{BurnCrank, SealPlatformCoin};
// anchor's #[program] expects the generated client-account modules at the crate root.
pub(crate) use burn::{__client_accounts_burn_crank, __client_accounts_seal_platform_coin};
#[cfg(feature = "cpi")] pub(crate) use burn::{__cpi_client_accounts_burn_crank, __cpi_client_accounts_seal_platform_coin};
#[path="buy.rs"]pub mod legacy_buy;
pub mod creator{#[path="collect_creator_fee.rs"]pub mod collect_creator_fee;}pub use creator::collect_creator_fee as collect_v2;
pub use collect_v2::CollectCreatorFee;
pub(crate) use collect_v2::__client_accounts_collect_creator_fee;
#[cfg(feature = "cpi")] pub(crate) use collect_v2::__cpi_client_accounts_collect_creator_fee;
#[path="legacy_native_payments.rs"]pub mod legacy_native_payments;
#[path="extend_account.rs"]pub mod extend;#[path = "fee_recipient.rs"] pub(crate) mod fee_recipient;
pub use extend::ExtendAccount;
pub(crate) use extend::__client_accounts_extend_account;
#[cfg(feature = "cpi")] pub(crate) use extend::__cpi_client_accounts_extend_account;
pub mod migrate;
pub use migrate::Migrate;
pub(crate) use migrate::__client_accounts_migrate;
#[cfg(feature = "cpi")] pub(crate) use migrate::__cpi_client_accounts_migrate;
#[path="sell.rs"]pub mod legacy_sell;
pub use legacy_sell::*;
#[path="util/constant_product.rs"] pub mod constant_product; #[path="util/fee.rs"] pub mod fee_math;#[path="util/payments.rs"] pub mod payments;
#[path="curve_quote_payments.rs"] pub mod quote_payments;
pub mod global_init;
pub mod create;
pub use create::*;

use anchor_lang::prelude::*;




// layout-pad: aligns the next Source site with the original lib.rs line












declare_id!("DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH");


// layout-pad: aligns the next Source site with the original lib.rs line

#[program]
pub mod glasspad_curve {
    use super::*;
    pub fn initialize(ctx:Context<Initialize>)->Result<()>{global_init::execute(ctx)}
// layout-pad: aligns the next Source site with the original lib.rs line

















// layout-pad: aligns the next Source site with the original lib.rs line

// layout-pad: aligns the next Source site with the original lib.rs line

















// layout-pad: aligns the next Source site with the original lib.rs line






























    pub fn create(ctx:Context<Create>,name:String,symbol:String,uri:String,creator:Pubkey)->Result<()>{
     require_keys_neq!(creator,Pubkey::default(),PumpError::CreatorShouldNotBeZero);
     {let a=&mut *ctx.accounts;let curve:&mut trade_types::BondingCurve=&mut a.bonding_curve;let g=&a.global;let quote_mint=Pubkey::default();let virtual_quote_reserves=if quote_mint==Pubkey::default(){g.initial_virtual_sol_reserves}else{g.initial_virtual_quote_reserves};curve.virtual_quote_reserves=virtual_quote_reserves;curve.initial_virtual_quote_reserves=virtual_quote_reserves;curve.virtual_token_reserves=g.initial_virtual_token_reserves;curve.real_token_reserves=g.initial_real_token_reserves;curve.real_quote_reserves=0;curve.token_total_supply=g.token_total_supply;curve.creator=creator;curve.quote_mint=quote_mint;curve.post_complete_base_out=0;curve.creator_fee_bps=0;curve.post_complete_quote_in=0;curve.depth=0;curve.is_mayhem_mode=false;curve.is_cashback_coin=false;curve.can_edit_creator_fee=false;curve.is_holder_reward=false;}let a=&ctx.accounts;
     create_metadata(&ctx,&name,&symbol,&uri,creator)?;
     mint_tokens(a,&a.mint,&a.associated_bonding_curve,ctx.bumps.mint_authority)?;
     revoke_mint_authority(ctx.bumps.mint_authority,&(&a.token_program,&a.mint,&a.mint_authority))?;
     let event=create_event(name,symbol,uri,creator,&a.mint,&a.user,&a.bonding_curve,&a.token_program,false,0)?;
     
     emit!(event);emit_cpi!(event);Ok(())
    }
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
    pub fn buy<'info>(ctx:Context<'_,'_,'_,'info,LegacyBuy<'info>>,amount:u64,max_sol_cost:u64,track_volume:OptionBool,partial_fill:OptionBool)->Result<()>{
     let fee_recipient=&ctx.accounts.fee_recipient;
     let bonding_curve=&mut ctx.accounts.bonding_curve;let (associated_bonding_curve,associated_user,token_program)=(&ctx.accounts.associated_bonding_curve,&ctx.accounts.associated_user,&ctx.accounts.token_program);
     crate::trade_types::prepare_bonding_curve(bonding_curve,&ctx.accounts.system_program,&ctx.accounts.user)?;
     crate::legacy_sell::fee_recipient::validate_fee_recipient(fee_recipient,&ctx.accounts.global,bonding_curve)?;
     let buyback_fee_recipient=crate::legacy_sell::buyback_fee_recipient(ctx.remaining_accounts,&ctx.accounts.global,1)?;
// layout-pad: aligns the next Source site with the original lib.rs line

















































































































































































































































     require!(!bonding_curve.complete,PumpError::BondingCurveComplete);
// layout-pad: aligns the next Source site with the original lib.rs line


     let amount=if partial_fill.0{amount.min(bonding_curve.real_token_reserves)}else{amount};
// layout-pad: aligns the next Source site with the original lib.rs line



     require_gt!(amount,0,PumpError::BuyZeroAmount);

     require_gte!(bonding_curve.real_token_reserves,amount,PumpError::NotEnoughTokensToBuy);
// layout-pad: aligns the next Source site with the original lib.rs line








     let sol_cost=u64::try_from(crate::legacy_sell::fee_cpi::buy_quote(bonding_curve,amount)?).map_err(|_|error!(PumpError::Truncation))?;
     let (protocol_fee,creator_fee,protocol_fee_bps,creator_fee_bps)=crate::fee_math::fees_for(bonding_curve,sol_cost)?;
// layout-pad: aligns the next Source site with the original lib.rs line




















     require_gte!(max_sol_cost,
// layout-pad: aligns the next Source site with the original lib.rs line








      sol_cost.checked_add(protocol_fee).ok_or_else(||error!(PumpError::Overflow))?

      .checked_add(creator_fee).ok_or_else(||error!(PumpError::Overflow))?,
      PumpError::TooMuchSolRequired);
// layout-pad: aligns the next Source site with the original lib.rs line





     bonding_curve.virtual_token_reserves=bonding_curve.virtual_token_reserves.checked_sub(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.real_token_reserves=bonding_curve.real_token_reserves.checked_sub(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.virtual_quote_reserves=bonding_curve.virtual_quote_reserves.checked_add(sol_cost).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.real_quote_reserves=bonding_curve.real_quote_reserves.checked_add(sol_cost).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line

     let transfer=|signer_seeds:&[&[&[u8]];1]|crate::payments::transfer_curve_tokens_ref(token_program,associated_bonding_curve,associated_user,&bonding_curve,ctx.accounts.mint.decimals,ctx.accounts.mint.as_ref(),amount,signer_seeds);transfer(&[&[b"bonding-curve",ctx.accounts.mint.key().as_ref(),&[ctx.bumps.bonding_curve]]])?;
// layout-pad: aligns the next Source site with the original lib.rs line








// layout-pad: aligns the next Source site with the original lib.rs line







// layout-pad: aligns the next Source site with the original lib.rs line












     let mint=ctx.accounts.mint.key();
     if creator_fee!=0{
      crate::legacy_sell::check_bonding_curve_v2(ctx.remaining_accounts,&mint.key())?;
       crate::legacy_buy::native_payments::ensure_rent_exempt(&ctx.accounts.system_program,&ctx.accounts.user,&ctx.accounts.creator_vault)?;
       crate::user_quote_payments::transfer_native_unchecked(ctx.accounts.system_program.as_ref(),&ctx.accounts.user,&ctx.accounts.creator_vault,creator_fee)?;
     }
     crate::user_quote_payments::transfer_native_curve(ctx.accounts.system_program.as_ref(),&ctx.accounts.user,&bonding_curve,sol_cost)?;
     let mut buyback_fee=0;
     if protocol_fee!=0{
      let (fee,remaining)=crate::split_protocol_fee(&ctx.accounts.global,protocol_fee,bonding_curve.is_mayhem_mode);
      if fee!=0{crate::user_quote_payments::transfer_native_curve_unchecked(ctx.accounts.system_program.as_ref(),&ctx.accounts.user,&buyback_fee_recipient,fee)?;}
      crate::user_quote_payments::transfer_native_recipient(ctx.accounts.system_program.as_ref(),&ctx.accounts.user,&fee_recipient,remaining)?;
      buyback_fee=fee;
     }
     let virtual_token_reserves=bonding_curve.virtual_token_reserves;
     let virtual_quote_reserves=bonding_curve.virtual_quote_reserves;
     let real_token_reserves=bonding_curve.real_token_reserves;
     let real_quote_reserves=bonding_curve.real_quote_reserves;
     let creator=bonding_curve.creator;
     let timestamp=Clock::get()?.unix_timestamp;
     let event=crate::trade_types::TradeEvent{
      total_unclaimed_tokens:0,
      total_claimed_tokens:0,
      current_sol_volume:0,
      last_update_timestamp:0,
      mint,
      sol_amount:sol_cost,
      token_amount:amount,
      is_buy:true,
      user:ctx.accounts.user.key(),
      timestamp,
      virtual_sol_reserves:virtual_quote_reserves,
      virtual_token_reserves,
      real_sol_reserves:real_quote_reserves,
      real_token_reserves,
      fee_recipient:fee_recipient.key(),
      fee_basis_points:protocol_fee_bps,
      fee:protocol_fee,
      creator,
      creator_fee_basis_points:creator_fee_bps,
      creator_fee,
      track_volume:track_volume.0,
      holder_rewards_bps:if bonding_curve.is_holder_reward{creator_fee_bps}else{0},
      holder_rewards:if bonding_curve.is_holder_reward{creator_fee}else{0},
      ix_name:"buy".to_string(),
      mayhem_mode:bonding_curve.is_mayhem_mode,
      cashback_fee_basis_points:0,
      cashback:0,
      buyback_fee_basis_points:if bonding_curve.is_mayhem_mode{0}else{ctx.accounts.global.buyback_basis_points},
      buyback_fee,
      shareholders:vec![],
      quote_mint:bonding_curve.quote_mint,
      quote_amount:sol_cost,
      virtual_quote_reserves,
      real_quote_reserves,
      creator_fee_unclaimed:bonding_curve.creator_fee,
      unparsed_trailing_field:0,
     };
     emit!(event);
     emit_cpi!(event);
     if ctx.accounts.bonding_curve.real_token_reserves==0{
      ctx.accounts.bonding_curve.complete=true;
      let event=crate::trade_types::CompleteEvent{user:ctx.accounts.user.key(),mint,bonding_curve:ctx.accounts.bonding_curve.key(),timestamp,quote_mint:ctx.accounts.bonding_curve.quote_mint};
      emit!(event);
      emit_cpi!(event);
     }
     Ok(())
    }
    pub fn buy_exact_sol_in<'info>(ctx:Context<'_,'_,'_,'info,LegacyBuy<'info>>,spendable_sol_in:u64,min_tokens_out:u64,track_volume:OptionBool,partial_fill:OptionBool)->Result<()>{legacy_buy::buy_exact_sol_in(ctx,spendable_sol_in,min_tokens_out,track_volume,partial_fill)}
    pub fn sell<'info>(ctx:Context<'_,'_,'info,'info,LegacySell<'info>>,amount:u64,min_sol_output:u64)->Result<()>{
     let fee_recipient=&ctx.accounts.fee_recipient;let user=&ctx.accounts.user;
     let bonding_curve=&mut ctx.accounts.bonding_curve;let (associated_bonding_curve,associated_user,token_program)=(&ctx.accounts.associated_bonding_curve,&ctx.accounts.associated_user,&ctx.accounts.token_program);
     crate::trade_types::prepare_bonding_curve(bonding_curve,&ctx.accounts.system_program,user)?;
     crate::legacy_sell::fee_recipient::validate_fee_recipient(fee_recipient,&ctx.accounts.global,bonding_curve)?;

// layout-pad: aligns the next Source site with the original lib.rs line
















































































































































     require!(!bonding_curve.complete,PumpError::BondingCurveComplete);
// layout-pad: aligns the next Source site with the original lib.rs line
     require_gt!(amount,0,PumpError::SellZeroAmount);
// layout-pad: aligns the next Source site with the original lib.rs line
     require_gte!(ctx.accounts.associated_user.amount,amount,PumpError::NotEnoughTokensToSell);
// layout-pad: aligns the next Source site with the original lib.rs line







     let sol_output=u64::try_from(bonding_curve.sell_quote(amount)?).map_err(|_|error!(PumpError::Truncation))?;
     let (protocol_fee,creator_fee,protocol_fee_bps,creator_fee_bps)=crate::fee_math::fees_for(bonding_curve,sol_output)?;
// layout-pad: aligns the next Source site with the original lib.rs line





















     let output=sol_output.checked_sub(protocol_fee).ok_or_else(||error!(PumpError::Overflow))?
// layout-pad: aligns the next Source site with the original lib.rs line
      .checked_sub(creator_fee).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line

     require_gte!(output,min_sol_output,PumpError::TooLittleSolReceived);
// layout-pad: aligns the next Source site with the original lib.rs line








     bonding_curve.virtual_token_reserves=bonding_curve.virtual_token_reserves.checked_add(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.real_token_reserves=bonding_curve.real_token_reserves.checked_add(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.virtual_quote_reserves=bonding_curve.virtual_quote_reserves.checked_sub(sol_output).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



     bonding_curve.real_quote_reserves=bonding_curve.real_quote_reserves.checked_sub(sol_output).ok_or_else(||error!(PumpError::Overflow))?;
     crate::collect_v2::transfer_user_tokens_ref(token_program,associated_user,associated_bonding_curve,&user,ctx.accounts.mint.decimals,ctx.accounts.mint.as_ref(),amount)?;
     let index=1;if creator_fee!=0{crate::legacy_sell::check_bonding_curve_v2(ctx.remaining_accounts,&ctx.accounts.mint.key())?;crate::legacy_buy::native_payments::ensure_rent_exempt(&ctx.accounts.system_program,&ctx.accounts.user,&ctx.accounts.creator_vault)?;crate::quote_payments::move_native(bonding_curve,&ctx.accounts.creator_vault,creator_fee)?;}
     let buyback_fee_recipient=crate::legacy_sell::buyback_fee_recipient(ctx.remaining_accounts,&ctx.accounts.global,index)?;
     crate::quote_payments::transfer_native(bonding_curve,&user,output)?;
     let mut buyback_fee=0;
     if protocol_fee!=0{let (fee,remaining)=crate::split_protocol_fee(&ctx.accounts.global,protocol_fee,bonding_curve.is_mayhem_mode);if fee!=0{crate::quote_payments::move_native(bonding_curve,&buyback_fee_recipient,fee)?;}crate::quote_payments::transfer_native(bonding_curve,&fee_recipient,remaining)?;buyback_fee=fee;}
     let virtual_token_reserves=bonding_curve.virtual_token_reserves;let virtual_quote_reserves=bonding_curve.virtual_quote_reserves;let real_token_reserves=bonding_curve.real_token_reserves;let real_quote_reserves=bonding_curve.real_quote_reserves;
     let creator=bonding_curve.creator;
     let event=crate::trade_types::TradeEvent{user:ctx.accounts.user.key(),mint:ctx.accounts.mint.key(),timestamp:Clock::get()?.unix_timestamp,total_unclaimed_tokens:0,total_claimed_tokens:0,current_sol_volume:0,last_update_timestamp:0,sol_amount:sol_output,token_amount:amount,is_buy:false,
      virtual_sol_reserves:virtual_quote_reserves,virtual_token_reserves,real_sol_reserves:real_quote_reserves,real_token_reserves,fee_recipient:fee_recipient.key(),fee_basis_points:protocol_fee_bps,fee:protocol_fee,creator,creator_fee_basis_points:creator_fee_bps,creator_fee,track_volume:false,
      holder_rewards_bps:if bonding_curve.is_holder_reward{creator_fee_bps}else{0},holder_rewards:if bonding_curve.is_holder_reward{creator_fee}else{0},ix_name:"sell".to_string(),mayhem_mode:bonding_curve.is_mayhem_mode,cashback_fee_basis_points:0,cashback:0,
      buyback_fee_basis_points:if bonding_curve.is_mayhem_mode{0}else{ctx.accounts.global.buyback_basis_points},buyback_fee,shareholders:vec![],quote_mint:bonding_curve.quote_mint,quote_amount:sol_output,virtual_quote_reserves,real_quote_reserves,creator_fee_unclaimed:bonding_curve.creator_fee,unparsed_trailing_field:0,};
     emit!(event);emit_cpi!(event);Ok(())
    }
    pub fn migrate<'info>(ctx:Context<'_,'_,'_,'info,Migrate<'info>>)->Result<()>{migrate::execute(ctx)}
    pub fn seal_platform_coin(ctx:Context<SealPlatformCoin>)->Result<()>{burn::seal_platform_coin(ctx)}
    pub fn burn_crank(ctx:Context<BurnCrank>)->Result<()>{burn::burn_crank(ctx)}
    pub fn extend_account(ctx:Context<ExtendAccount>)->Result<()>{extend::execute(ctx)}
    pub fn collect_creator_fee(ctx:Context<CollectCreatorFee>)->Result<()>{collect_v2::collect_creator_fee(ctx)









    }




}

#[error_code]
pub enum PumpError {
    #[msg("The given account is not authorized to execute this instruction.")]
    NotAuthorized,
    #[msg("The program is already initialized.")]
    AlreadyInitialized,
    #[msg("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired,
    #[msg("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived,
    #[msg("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve,
    #[msg("the curve is complete. anyone can call migrate to open its raydium pool.")]
    BondingCurveComplete,
    #[msg("The bonding curve has not completed.")]
    BondingCurveNotComplete,
    #[msg("The program is not initialized.")]
    NotInitialized,
    #[msg("Withdraw too frequent")]
    WithdrawTooFrequent,
    #[msg("new_size should be > current_size")]
    NewSizeShouldBeGreaterThanCurrentSize,
    #[msg("Account type not supported")]
    AccountTypeNotSupported,
    #[msg("initial_real_token_reserves should be less than token_total_supply")]
    InitialRealTokenReservesShouldBeLessThanTokenTotalSupply,
    #[msg("initial_virtual_token_reserves should be greater than initial_real_token_reserves")]
    InitialVirtualTokenReservesShouldBeGreaterThanInitialRealTokenReserves,
    #[msg("fee_basis_points greater than maximum")]
    FeeBasisPointsGreaterThanMaximum,
    #[msg("Withdraw authority cannot be set to System Program ID")]
    AllZerosWithdrawAuthority,
    #[msg("pool_migration_fee should be less than final_real_sol_reserves")]
    PoolMigrationFeeShouldBeLessThanFinalRealSolReserves,
    #[msg("pool_migration_fee should be greater than creator_fee + MAX_MIGRATE_FEES")]
    PoolMigrationFeeShouldBeGreaterThanCreatorFeePlusMaxMigrateFees,
    #[msg("Migrate instruction is disabled")]
    DisabledWithdraw,
    #[msg("Migrate instruction is disabled")]
    DisabledMigrate,
    #[msg("Invalid creator pubkey")]
    InvalidCreator,
    #[msg("Buy zero amount")]
    BuyZeroAmount,
    #[msg("Not enough tokens to buy")]
    NotEnoughTokensToBuy,
    #[msg("Sell zero amount")]
    SellZeroAmount,
    #[msg("Not enough tokens to sell")]
    NotEnoughTokensToSell,
    #[msg("Overflow")]
    Overflow,
    #[msg("Truncation")]
    Truncation,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Not enough remaining accounts")]
    NotEnoughRemainingAccounts,
    #[msg("All fee recipients should be non-zero")]
    AllFeeRecipientsShouldBeNonZero,
    #[msg("Unsorted or not unique fee recipients")]
    UnsortedNotUniqueFeeRecipients,
    #[msg("Creator should not be zero")]
    CreatorShouldNotBeZero,
    StartTimeInThePast,
    EndTimeInThePast,
    EndTimeBeforeStartTime,
    TimeRangeTooLarge,
    EndTimeBeforeCurrentDay,
    SupplyUpdateForFinishedRange,
    DayIndexAfterEndIndex,
    DayInActiveRange,
    InvalidIncentiveMint,
    #[msg("Buy: Not enough SOL to cover for rent exemption.")]
    BuyNotEnoughSolToCoverRent,
    #[msg("Buy: Not enough SOL to cover for fees.")]
    BuyNotEnoughSolToCoverFees,
    #[msg("Slippage: Would buy less tokens than expected min_tokens_out")]
    BuySlippageBelowMinTokensOut,
    NameTooLong,
    SymbolTooLong,
    UriTooLong,
    CreateV2Disabled,
    CpitializeMayhemFailed,
    MayhemModeDisabled,
    #[msg("creator has been migrated to sharing config")]
    CreatorMigratedToSharingConfig,
    #[msg("creator vault was moved to a sharing config")]
    UnableToDistributeCreatorVaultMigratedToSharingConfig,
    #[msg("Sharing config is not active")]
    SharingConfigNotActive,
    #[msg("The recipient account is executable, so it cannot receive lamports, remove it from the team first")]
    UnableToDistributeCreatorFeesToExecutableRecipient,
    #[msg("Bonding curve creator does not match sharing config")]
    BondingCurveAndSharingConfigCreatorMismatch,
    #[msg("Remaining accounts do not match shareholders, make sure to pass exactly the same pubkeys in the same order")]
    ShareholdersAndRemainingAccountsMismatch,
    #[msg("Share bps must be greater than 0")]
    InvalidShareBps,
    #[msg("Cashback is not enabled")]
    CashbackNotEnabled,
    #[msg("Buyback fee recipient not authorized")]
    BuybackFeeRecipientNotAuthorized,
    AllBuybackFeeRecipientsShouldBeNonZero,
    NotUniqueBuybackFeeRecipients,
    #[msg("buyback_basis_points must be <= 10_000")]
    BuybackBasisPointsOutOfRange,
    #[msg("buyback fee recipients require exactly 8 remaining accounts (or none)")]
    WrongBuybackFeeRecipientsCount,
    BuybackFeeRecipientMissing,
    #[msg("Unsupported quote mint")]
    UnsupportedQuoteMint,
    #[msg("Create v2: quote token program must be SPL Token or Token-2022")]
    InvalidQuoteTokenProgram,
    #[msg("Create v2: associated quote bonding curve address does not match derivation")]
    InvalidAssociatedQuoteBondingCurve,
    #[msg("Quote mint whitelist is full")]
    QuoteMintWhitelistFull,
    #[msg("Quote mint is already whitelisted")]
    QuoteMintAlreadyWhitelisted,
    #[msg("Quote mint is not in the whitelist")]
    QuoteMintNotWhitelisted,
    #[msg("Quote mint cannot be added or removed via whitelist (default or native SOL mint)")]
    QuoteMintNotEligibleForWhitelist,
    #[msg("Unable to distribute creator fees to uninitialized account")]
    UnableToDistributeCreatorFeesToUninitializedAccount,
    #[msg("Mayhem mode quote mint not allowed")]
    MayhemModeQuoteMintNotAllowed,
    #[msg("Cashback trade is missing the required remaining accounts")]
    MissingCashbackAccounts,
    #[msg("Cashback user_volume_accumulator account is invalid")]
    InvalidCashbackAccumulator,
    #[msg("bonding_curve_v2 remaining account is missing or invalid")]
    InvalidBondingCurveV2,
    #[msg("quote_control remaining account does not match derivation or is uninitialized")]
    InvalidQuoteControl,
    #[msg("Cashback recipient token account is not owned by user")]
    InvalidCashbackRecipient,
    #[msg("Configurable creator fees are disabled")]
    CreatorFeeNotConfigurable,
    #[msg("Creator fee basis points must be between 1 and the configured maximum")]
    CreatorFeeBpsOutOfRange,
    #[msg("Creator fee is not editable for this bonding curve")]
    CreatorFeeNotEditable,
    #[msg("Creator fee cannot be configured for a cashback coin")]
    CreatorFeeNotAllowedForCashbackCoin,
    #[msg("Bonding curve has already migrated")]
    BondingCurveAlreadyMigrated,
    #[msg("Cashback coins can no longer be created")]
    CashbackDeprecated,
    #[msg("The creator of a holder-reward coin cannot be changed")]
    HolderRewardCreatorImmutable,
    #[msg("Holder-reward coins are disabled")]
    HolderRewardDisabled,
    #[msg("Holder-reward amounts and recipient accounts do not match")]
    HolderRewardRecipientsMismatch,
    #[msg("The holder-rewards PDA cannot be left below its rent-exempt minimum")]
    HolderRewardsRentFloor,
    #[msg("A holder-rewards token account is required on a token quote")]
    HolderRewardTokenAccountMissing,
    #[msg("CTO is not allowed on a mayhem-mode coin")]
    CtoNotAllowedForMayhemCoin,
    #[msg("new_creator is required unless converting to holder rewards")]
    CtoNewCreatorRequired,
    #[msg("new_creator must be omitted when converting to holder rewards")]
    CtoNewCreatorNotAllowed,
    #[msg("Creator fee is not configurable on a SOL or whitelisted quote; the fee schedule applies")]
    CreatorFeeNotConfigurableForQuote,
    #[msg("current_creator must be passed writable so the outgoing creator can be paid")]
    CtoCreatorAccountNotWritable,
    #[msg("A frozen sharing-config vault account holds a balance; thaw it before the holder conversion")]
    CtoSharedVaultFrozen,
    #[msg("v3 trades do not support cashback coins; use the v2 instructions")]
    CashbackCoinNotSupported,
    #[msg("creator_fee must be swept before the creator changes or the vault is distributed")]
    CreatorFeesNotSwept,
    #[msg("The fee config tier table is empty")]
    FeeTiersEmpty,
    #[msg("The fee config account is shorter than its initialize size")]
    FeeConfigTooShort,
    #[msg("multi-hop: a hop's input mint is neither side of its bonding curve")]
    MultiHopDiscontinuousPath,
    #[msg("Quote bonding curve remaining account does not match derivation or is uninitialized")]
    InvalidQuoteBondingCurve,
    #[msg("Quote bonding curve is not eligible as a quote: mayhem mode, or the chain's depth-0 root is quoted in a mint no list admits")]
    QuoteBondingCurveNotEligible,
    #[msg("A quote-chain bonding curve has migrated: its pool and vault remaining accounts are required")]
    QuotePoolAccountsRequired,
    #[msg("quote pool remaining account is not an initialized amm pool")]
    QuotePoolNotFound,
    #[msg("Quote pool or vault remaining accounts do not match derivation or are invalid")]
    InvalidQuotePool,
    #[msg("Derived quote reserves are zero")]
    QuoteReservesOutOfRange,
    #[msg("The new curve's depth (quote coin depth plus one) exceeds Global.max_curve_depth or the program's cap of 5")]
    CurveDepthExceeded,
    #[msg("multi-hop: hop_fees must carry one flag byte per curve hop using only the protocol and creator bits")]
    MultiHopHopFeesMismatch,
    #[msg("A quote-chain coin's curve is complete but not yet migrated; retry once migrate_v2 creates its pool")]
    QuoteCurveAwaitingMigration,
    #[msg("multi-hop: mayhem-mode curves are not routable; use the v2 instructions")]
    MultiHopMayhemCurveNotSupported,
    #[msg("The quote coin is nested: a bonding curve remaining account is required for every coin on its quote chain down to the depth-0 root")]
    QuoteChainAccountsRequired,
    // Binary evidence extends the captured official IDL, which stops at 6109.
    #[msg("quote_mint must be writable for a quote with a Token-2022 transfer fee: its withheld fees are harvested to the mint")]
    QuoteMintNotWritable = 110,
}

// layout-pad: aligns the next Source site with the original lib.rs line







































































































































































































































































































































































































































































































#[account]
pub struct Global {
    pub initialized: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [Pubkey; 7],
    pub set_creator_authority: Pubkey,
    pub admin_set_creator_authority: Pubkey,
    pub create_v2_enabled: bool,
    pub whitelist_pda: Pubkey,
    pub reserved_fee_recipient: Pubkey,
    pub mayhem_mode_enabled: bool,
    pub reserved_fee_recipients: [Pubkey; 7],
    pub is_cashback_enabled: bool,
    pub buyback_fee_recipients: [Pubkey; 8],
    pub buyback_basis_points: u64,
    pub initial_virtual_quote_reserves: u64,
    pub whitelisted_quote_mints: [Pubkey; 1],
    pub creator_fee_configurable: bool,
    pub max_configurable_creator_fee_bps: u64,
    pub platform_coin: Pubkey,
    pub is_holder_reward_enabled: bool,
    pub max_curve_depth: u8,
}

// layout-pad: aligns the next Source site with the original lib.rs line

















impl Global {
 #[inline(never)]
 pub fn set_buyback_basis_points(&mut self,value:u64)->Result<()>{
  require!(value<=10000,PumpError::BuybackBasisPointsOutOfRange);
  self.buyback_basis_points=value;
  Ok(())
 }
// layout-pad: aligns the next Source site with the original lib.rs line


 #[inline(never)]
 pub fn validate_buyback_fee_recipient(&self,buyback_fee_recipient:&Pubkey)->Result<()>{
  require_keys_neq!(*buyback_fee_recipient,Pubkey::default(),PumpError::BuybackFeeRecipientMissing);
// layout-pad: aligns the next Source site with the original lib.rs line



  require!(self.buyback_fee_recipients.contains(buyback_fee_recipient),PumpError::BuybackFeeRecipientNotAuthorized);
  Ok(())
 }
// layout-pad: aligns the next Source site with the original lib.rs line





 #[inline(never)]
 pub fn set_buyback_fee_recipients(&mut self,recipients:[Pubkey;8])->Result<()>{
  require!(recipients.iter().all(|key|*key!=Pubkey::default()),PumpError::AllBuybackFeeRecipientsShouldBeNonZero);
// layout-pad: aligns the next Source site with the original lib.rs line








  for left in 0..8 {for right in left+1..8 {require_neq!(recipients[left],recipients[right],PumpError::NotUniqueBuybackFeeRecipients);}}
  self.buyback_fee_recipients=recipients;
  Ok(())
 }
}

// layout-pad: aligns the next Source site with the original lib.rs line




















/// Buyback / retained split of a protocol fee (no buyback in mayhem mode).
#[inline(always)]
fn protocol_fee_split(buyback_basis_points: u64, protocol_fee: u64, is_mayhem_mode: bool) -> (u64, u64) {
    if is_mayhem_mode { return (0, protocol_fee); } let scaled = buyback_basis_points as u128 * protocol_fee as u128 / 10000;
    let buyback_fee = u64::try_from(scaled).expect("buyback fee is at most protocol_fee");
    let remaining = protocol_fee
        .checked_sub(buyback_fee)
            .expect("buyback_fee never exceeds protocol_fee");
    (buyback_fee, remaining)
}
#[inline(never)]
pub(crate) fn split_protocol_fee(global: &crate::Global, protocol_fee: u64, is_mayhem_mode: bool) -> (u64, u64) {
    protocol_fee_split(global.buyback_basis_points, protocol_fee, is_mayhem_mode)
}


impl Global {
    pub fn add_quote_mint(&mut self,quote_mint:Pubkey)->Result<()>{
     require_keys_neq!(quote_mint,Pubkey::default(),PumpError::QuoteMintNotEligibleForWhitelist);
// layout-pad: aligns the next Source site with the original lib.rs line



     require_keys_neq!(quote_mint,anchor_spl::token::spl_token::native_mint::ID,PumpError::QuoteMintNotEligibleForWhitelist);
// layout-pad: aligns the next Source site with the original lib.rs line



     require_eq!(self.whitelisted_quote_mints.contains(&quote_mint),false,PumpError::QuoteMintAlreadyWhitelisted);
// layout-pad: aligns the next Source site with the original lib.rs line









     for slot in self.whitelisted_quote_mints.iter_mut(){if *slot==Pubkey::default(){*slot=quote_mint;return Ok(());}} err!(PumpError::QuoteMintWhitelistFull)
        }
// layout-pad: aligns the next Source site with the original lib.rs line
    pub fn remove_quote_mint(&mut self,quote_mint:Pubkey)->Result<()>{
     require_keys_neq!(quote_mint,Pubkey::default(),PumpError::QuoteMintNotEligibleForWhitelist);
// layout-pad: aligns the next Source site with the original lib.rs line









     for slot in self.whitelisted_quote_mints.iter_mut(){if *slot==quote_mint{*slot=Pubkey::default();return Ok(());}} err!(PumpError::QuoteMintNotWhitelisted)
        }
}
pub mod trade_types;
/// Checked normal-buy quote calculation precedes the fee CPI.
pub fn buy_quote(curve: &trade_types::BondingCurve, amount: u64) -> Result<u128> {
    let numerator = u128::from(amount) * u128::from(curve.virtual_quote_reserves);
    let denominator = u128::from(curve.virtual_token_reserves).checked_sub(u128::from(amount))
// layout-pad: aligns the next Source site with the original lib.rs line

















































































































































































































































































































































































































































        .ok_or_else(|| error!(crate::PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line
    require!(denominator != 0, crate::PumpError::DivisionByZero);
    Ok(numerator / denominator + 1)
}

impl trade_types::BondingCurve {
 #[inline(never)]
 pub fn sell_quote(&self,amount:u64)->Result<u128>{
  let numerator=self.virtual_quote_reserves as u128*amount as u128;
// layout-pad: aligns the next Source site with the original lib.rs line















  numerator.checked_div(self.virtual_token_reserves as u128+amount as u128).ok_or_else(||error!(PumpError::DivisionByZero))
 }
}
#[inline(never)] pub fn tokens_for_quote(curve:&crate::trade_types::BondingCurve,quote:u64)->Result<u128>{let effective=quote.checked_sub(1).ok_or(PumpError::Overflow)?;crate::constant_product::output(effective as u128,curve.virtual_quote_reserves as u128,curve.virtual_token_reserves as u128)}

// layout-pad: aligns the next Source site with the original lib.rs line








impl trade_types::BondingCurve {
 #[inline(never)]
 pub fn apply_buy(&mut self,amount:u64,quote:u64)->Result<()>{
  self.virtual_token_reserves=self.virtual_token_reserves.checked_sub(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.real_token_reserves=self.real_token_reserves.checked_sub(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.virtual_quote_reserves=self.virtual_quote_reserves.checked_add(quote).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.real_quote_reserves=self.real_quote_reserves.checked_add(quote).ok_or_else(||error!(PumpError::Overflow))?;
  Ok(())
 }
// layout-pad: aligns the next Source site with the original lib.rs line



 #[inline(never)]
 pub fn apply_sell(&mut self,amount:u64,quote:u64)->Result<()>{
  self.virtual_token_reserves=self.virtual_token_reserves.checked_add(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.real_token_reserves=self.real_token_reserves.checked_add(amount).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.virtual_quote_reserves=self.virtual_quote_reserves.checked_sub(quote).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line



  self.real_quote_reserves=self.real_quote_reserves.checked_sub(quote).ok_or_else(||error!(PumpError::Overflow))?;
  Ok(())
 }
}

impl trade_types::BondingCurve {
 #[inline(never)]
 pub fn accrue_fees(&mut self,global:&Global,protocol_fee:u64,creator_fee:u64)->Result<u64>{
  let (buyback_fee,retained)=protocol_fee_split(global.buyback_basis_points,protocol_fee,self.is_mayhem_mode);



// layout-pad: aligns the next Source site with the original lib.rs line


  self.protocol_fees=self.protocol_fees.checked_add(retained).ok_or_else(||error!(PumpError::Overflow))?;
// layout-pad: aligns the next Source site with the original lib.rs line


  self.creator_fee=self.creator_fee.checked_add(creator_fee).ok_or_else(||error!(PumpError::Overflow))?;
  Ok(buyback_fee)
 }
}

#[inline(never)]
pub fn reserved_bonding_curve_fees(curve: &trade_types::BondingCurve) -> Result<u64> {
    curve.creator_fee.checked_add(curve.protocol_fees)
        .ok_or_else(|| error!(crate::PumpError::Overflow))
}























#[event]
#[derive(Clone)]
pub struct CreateEvent {pub name:String,pub symbol:String,pub uri:String,pub mint:Pubkey,pub bonding_curve:Pubkey,pub user:Pubkey,pub creator:Pubkey,pub timestamp:i64,pub virtual_token_reserves:u64,pub virtual_sol_reserves:u64,pub real_token_reserves:u64,pub token_total_supply:u64,pub token_program:Pubkey,pub is_mayhem_mode:bool,pub is_cashback_enabled:bool,pub quote_mint:Pubkey,pub virtual_quote_reserves:u64,pub creator_fee_bps:u64,pub is_holder_reward:bool,pub depth:u8}






















#[event]
#[derive(Clone)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator: Pubkey,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub ix_name: String,
    pub mayhem_mode: bool,
    pub cashback_fee_basis_points: u64,
    pub cashback: u64,
    pub buyback_fee_basis_points: u64,
    pub buyback_fee: u64,
    pub shareholders: Vec<Shareholder>,
    pub quote_mint: Pubkey,
    pub quote_amount: u64,
    pub virtual_quote_reserves: u64,
    pub real_quote_reserves: u64,
    pub holder_rewards_bps: u64,
    pub holder_rewards: u64,
    pub creator_fee_unclaimed: u64,
    pub unparsed_trailing_field: u64,
}

#[event]
#[derive(Clone)]
pub struct CompleteEvent {pub user:Pubkey,pub mint:Pubkey,pub bonding_curve:Pubkey,pub timestamp:i64,pub quote_mint:Pubkey}

































































#[derive(Accounts)]
pub struct Initialize<'info>{
 #[account(init,payer=user,space=1088,seeds=[b"global"],bump)]
 pub global:Account<'info,Global>,
 #[account(mut)]pub user:Signer<'info>,
 pub system_program:Program<'info,System>,
}

// layout-pad: aligns the next Source site with the original lib.rs line















pub mod bonding_curve_instruction_accounts{use super::*;#[event_cpi]
#[derive(Accounts)]
pub struct Create<'info>{
 #[account(init,payer=user,mint::decimals=6,mint::authority=mint_authority)]
 pub mint:Box<Account<'info,anchor_spl::token::Mint>>,
 /// CHECK: canonical program mint authority.
 #[account(seeds=[b"mint-authority"],bump)]
 pub mint_authority:UncheckedAccount<'info>,
 #[account(init,payer=user,space=166,seeds=[b"bonding-curve",mint.key().as_ref()],bump)]
 pub bonding_curve:Box<Account<'info,trade_types::BondingCurve>>,
 #[account(init,payer=user,associated_token::mint=mint,associated_token::authority=bonding_curve)]
 pub associated_bonding_curve:Box<Account<'info,anchor_spl::token::TokenAccount>>,
 #[account(seeds=[b"global"],bump)]
 pub global:Account<'info,Global>,
 pub mpl_token_metadata:Program<'info,create::MetadataProgram>,
 /// CHECK: canonical metadata account, allocated by the metadata program.
 #[account(mut,seeds=[b"metadata",anchor_spl::metadata::ID.as_ref(),mint.key().as_ref()],bump,seeds::program=mpl_token_metadata.key())]
 pub metadata:UncheckedAccount<'info>,
 #[account(mut)]pub user:Signer<'info>,
 pub system_program:Program<'info,System>,
 pub token_program:Program<'info,anchor_spl::token::Token>,
 pub associated_token_program:Program<'info,anchor_spl::associated_token::AssociatedToken>,
 pub rent:Sysvar<'info,Rent>,
}

// layout-pad: aligns the next Source site with the original lib.rs line



































#[derive(Accounts)]
pub struct LegacyBuy<'info>{
 #[account(seeds=[b"global"],bump)]pub global:Box<Account<'info,Global>>,
 /// CHECK: handler validates recipient.
 #[account(mut)]pub fee_recipient:UncheckedAccount<'info>,
 pub mint:InterfaceAccount<'info,anchor_spl::token_interface::Mint>,
 #[account(mut,seeds=[b"bonding-curve",mint.key().as_ref()],bump,constraint=bonding_curve.quote_mint==Pubkey::default() @ PumpError::UnsupportedQuoteMint)]pub bonding_curve:Account<'info,trade_types::BondingCurve>,
 #[account(mut,associated_token::mint=mint,associated_token::authority=bonding_curve,associated_token::token_program=token_program)]pub associated_bonding_curve:InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,
 #[account(mut,token::mint=mint)]pub associated_user:InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,
 #[account(mut)]pub user:Signer<'info>,
 pub system_program:Program<'info,System>,
 pub token_program:Interface<'info,anchor_spl::token_interface::TokenInterface>,
 /// CHECK: PDA checked.
 #[account(mut,seeds=[b"creator-vault",bonding_curve.creator.as_ref()],bump)]pub creator_vault:AccountInfo<'info>,
 /// CHECK: event authority.
 #[account(seeds=[b"__event_authority"],bump)]pub event_authority:AccountInfo<'info>,
 /// CHECK: self CPI.
 pub program:Program<'info,crate::program::GlasspadCurve>,
}

#[derive(Accounts)]
pub struct LegacySell<'info>{
 #[account(seeds=[b"global"],bump)]pub global:Box<Account<'info,Global>>,#[doc="CHECK: handler validates recipient."]#[account(mut)]pub fee_recipient:UncheckedAccount<'info>,pub mint:InterfaceAccount<'info,anchor_spl::token_interface::Mint>,#[account(mut,seeds=[b"bonding-curve",mint.key().as_ref()],bump,constraint=bonding_curve.quote_mint==Pubkey::default() @ PumpError::UnsupportedQuoteMint)]pub bonding_curve:Account<'info,trade_types::BondingCurve>,
 #[account(mut,associated_token::mint=mint,associated_token::authority=bonding_curve,associated_token::token_program=token_program)]pub associated_bonding_curve:InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,#[account(mut,token::mint=mint)]pub associated_user:InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,#[account(mut)]pub user:Signer<'info>,pub system_program:Program<'info,System>,#[doc="CHECK: PDA checked."]#[account(mut,seeds=[b"creator-vault",bonding_curve.creator.as_ref()],bump)]pub creator_vault:AccountInfo<'info>,pub token_program:Interface<'info,anchor_spl::token_interface::TokenInterface>,
 #[doc="CHECK: event authority."]#[account(seeds=[b"__event_authority"],bump)]pub event_authority:AccountInfo<'info>,#[doc="CHECK: self CPI."]pub program:Program<'info,crate::program::GlasspadCurve>,
}}pub use bonding_curve_instruction_accounts::*;



#[derive(AnchorSerialize,Clone)]
pub struct OptionBool(pub bool);
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad


























































// Shared user-signed quote payment helpers.
pub mod user_quote_payments {

pub use crate::payments::uqp_transfer_native as transfer_native;
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

pub use crate::payments::uqp_transfer_native_unchecked as transfer_native_unchecked;
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

pub use crate::payments::uqp_transfer_native_curve_unchecked as transfer_native_curve_unchecked;
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
pub use crate::payments::uqp_transfer_native_owned as transfer_native_owned;
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
pub use crate::payments::uqp_transfer_native_curve as transfer_native_curve;
pub use crate::payments::uqp_transfer_native_recipient as transfer_native_recipient;
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad
// layout-pad

}

use crate::trade_types::Shareholder;













































fn create_event<'info>(name:String,symbol:String,uri:String,creator:Pubkey,mint:&Account<'info,anchor_spl::token::Mint>,user:&Signer<'info>,bonding_curve:&Account<'info,trade_types::BondingCurve>,token_program:&Program<'info,anchor_spl::token::Token>,is_mayhem_mode:bool,creator_fee_bps:u64)->Result<CreateEvent>{
 let virtual_token_reserves=bonding_curve.virtual_token_reserves;let virtual_sol_reserves=bonding_curve.virtual_quote_reserves;let real_token_reserves=bonding_curve.real_token_reserves;let token_total_supply=bonding_curve.token_total_supply;let is_holder_reward=bonding_curve.is_holder_reward;let depth=bonding_curve.depth;let quote_mint=bonding_curve.quote_mint;Ok(CreateEvent{name,symbol,uri,mint:mint.key(),bonding_curve:bonding_curve.key(),user:user.key(),creator,timestamp:Clock::get()?.unix_timestamp,virtual_token_reserves,virtual_sol_reserves,real_token_reserves,token_total_supply,token_program:token_program.key(),is_mayhem_mode,is_cashback_enabled:false,quote_mint:quote_mint,virtual_quote_reserves:virtual_sol_reserves,creator_fee_bps,is_holder_reward,depth})
}
#[inline(always)]
fn create_metadata(ctx:&Context<Create>,name:&String,symbol:&String,uri:&String,creator:Pubkey)->Result<()>{
     let a=&ctx.accounts;
     let (name,symbol,uri)=(name.clone(),symbol.clone(),uri.clone());let bump=ctx.bumps.mint_authority;let mint:&Account<anchor_spl::token::Mint>=&a.mint;let signer_seeds:&[&[&[u8]]]=&[&[b"mint-authority",&[bump]]];metadata_cpi(&(&a.mpl_token_metadata,&a.metadata,mint,&a.mint_authority,&a.user,&a.system_program,&a.rent),(name,symbol,uri),creator,signer_seeds)?;
     Ok(())
}
fn metadata_cpi<'info>(accounts:&(&Program<'info,create::MetadataProgram>,&UncheckedAccount<'info>,&Account<'info,anchor_spl::token::Mint>,&UncheckedAccount<'info>,&Signer<'info>,&Program<'info,System>,&Sysvar<'info,Rent>),metadata:(String,String,String),creator:Pubkey,signer_seeds:&[&[&[u8]]])->Result<()>{
     let (name,symbol,uri)=metadata;
     anchor_spl::metadata::create_metadata_accounts_v3(
      CpiContext::new_with_signer(accounts.0.to_account_info(),anchor_spl::metadata::CreateMetadataAccountsV3{
       metadata:accounts.1.to_account_info(),mint:accounts.2.to_account_info(),mint_authority:accounts.3.to_account_info(),payer:accounts.4.to_account_info(),update_authority:accounts.3.to_account_info(),system_program:accounts.5.to_account_info(),rent:accounts.6.to_account_info(),
      },signer_seeds),
      anchor_spl::metadata::mpl_token_metadata::types::DataV2{name,symbol,uri,seller_fee_basis_points:0,creators:Some(vec![anchor_spl::metadata::mpl_token_metadata::types::Creator{address:creator,verified:false,share:100}]),collection:None,uses:None},
      false,true,None,
     )?;
     Ok(())
}
fn mint_tokens<'info>(a:&Create<'info>,mint:&Box<Account<'info,anchor_spl::token::Mint>>,to:&Box<Account<'info,anchor_spl::token::TokenAccount>>,bump:u8)->Result<()>{
     let signer_seeds:&[&[&[u8]]]=&[&[b"mint-authority",&[bump]]];anchor_spl::token::mint_to(CpiContext::new_with_signer(a.token_program.to_account_info(),anchor_spl::token::MintTo{mint:mint.to_account_info(),to:to.to_account_info(),authority:a.mint_authority.to_account_info()},signer_seeds),a.global.token_total_supply)?;
     Ok(())
}
fn revoke_mint_authority<'info>(bump:u8,accounts:&(&Program<'info,anchor_spl::token::Token>,&Account<'info,anchor_spl::token::Mint>,&UncheckedAccount<'info>))->Result<()>{
     let signer_seeds:&[&[&[u8]]]=&[&[b"mint-authority",&[bump]]];anchor_spl::token::set_authority(CpiContext::new_with_signer(accounts.0.to_account_info(),anchor_spl::token::SetAuthority{account_or_mint:accounts.1.to_account_info(),current_authority:accounts.2.to_account_info()},signer_seeds),anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,None)?;
     Ok(())
}
