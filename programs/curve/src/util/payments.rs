use anchor_lang::prelude::*;use anchor_spl::token_interface::{self,TokenAccount,TokenInterface,TransferChecked};
use crate::*;use crate::trade_types::BondingCurve;

impl AnchorDeserialize for OptionBool {
 fn deserialize_reader<R:std::io::Read>(reader:&mut R)->std::io::Result<Self>{
  let mut b=[0u8];match reader.read_exact(&mut b){Err(e) if e.kind()==std::io::ErrorKind::UnexpectedEof=>Ok(Self(false)),Err(e)=>Err(e),Ok(())=>match b[0]{0=>Ok(Self(false)),1=>Ok(Self(true)),value=>Err(std::io::Error::new(std::io::ErrorKind::InvalidData,format!("Invalid bool representation: {}",value)))}}
 }
}
    #[inline(never)]
    pub fn uqp_transfer_native_unchecked<'info>(
        system_program: &AccountInfo<'info>,
        payer: &Signer<'info>,
        destination: &AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: destination.clone(),
            }), amount,
        )
    }
#[inline(never)]
pub fn migrate_transfer_native<'info>(system_program:&AccountInfo<'info>,source:&UncheckedAccount<'info>,destination:&AccountInfo<'info>,amount:u64,signer_seeds:&[&[&[u8]];1])->Result<()>{
 if amount==0{return Ok(());}
 anchor_lang::system_program::transfer(CpiContext::new_with_signer(system_program.clone(),anchor_lang::system_program::Transfer{from:source.to_account_info(),to:destination.clone()},signer_seeds),amount)
}
#[inline(never)]
pub fn legacy_transfer<'info>(
    system_program: &AccountInfo<'info>,
    payer: &Signer<'info>,
    destination: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    anchor_lang::system_program::transfer(
            CpiContext::new(
                system_program.clone(),
                anchor_lang::system_program::Transfer {
                    from: payer.to_account_info(),
                    to: destination.clone(),
                },
            ),
            amount,
        )
}
    #[inline(never)] pub fn uqp_transfer_native_curve<'info>(system_program: &AccountInfo<'info>, payer: &Signer<'info>, destination: &&mut Account<'info, BondingCurve>, amount: u64) -> Result<()> { if amount == 0 { return Ok(()); } anchor_lang::system_program::transfer(CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer { from: payer.to_account_info(), to: destination.to_account_info() }), amount) }
    #[inline(never)]
    pub fn uqp_transfer_native_recipient<'info>(
        system_program: &AccountInfo<'info>,
        payer: &Signer<'info>,
        destination: &&UncheckedAccount<'info>,
        amount: u64,
    ) -> Result<()> {
        if amount == 0 { return Ok(()); }
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: destination.to_account_info(),
            }), amount,
        )
    }
#[inline(never)]
pub(crate) fn collect_transfer_native<'info>(
 system_program:&AccountInfo<'info>,source:&AccountInfo<'info>,destination:&AccountInfo<'info>,
 amount:u64,signer_seeds:&[&[&[u8]]],
)->Result<()>{
 if amount==0{return Ok(());}
 anchor_lang::system_program::transfer(
  CpiContext::new_with_signer(system_program.clone(),anchor_lang::system_program::Transfer{from:source.clone(),to:destination.clone()},signer_seeds),
  amount,
 )
}
    #[inline(never)]
    pub fn uqp_transfer_native_owned<'info>(
        system_program: &AccountInfo<'info>,
        payer: &Signer<'info>,
        destination: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: destination.clone(),
            }), amount,
        )
    }
    #[inline(never)]
    pub fn uqp_transfer_native<'info>(
        system_program: &AccountInfo<'info>,
        payer: &Signer<'info>,
        destination: &Account<'info, BondingCurve>,
        amount: u64,
    ) -> Result<()> {
        if amount == 0 { return Ok(()); }
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: destination.to_account_info(),
            }), amount,
        )
    }
    #[inline(never)]
    pub fn uqp_transfer_native_curve_unchecked<'info>(
        system_program: &AccountInfo<'info>,
        payer: &Signer<'info>,
        destination: &&AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program.clone(), anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: destination.to_account_info(),
            }), amount,
        )
    }
pub fn collect_transfer_tokens<'info>(
 token_program:&Interface<'info,anchor_spl::token_interface::TokenInterface>,
 source:&AccountInfo<'info>,destination:&AccountInfo<'info>,authority:&AccountInfo<'info>,
 decimals:u8,mint:&AccountInfo<'info>,amount:u64,signer_seeds:&[&[&[u8]];1],
)->Result<()>{
 if amount==0{return Ok(());}
 anchor_spl::token_interface::transfer_checked(
  CpiContext::new_with_signer(token_program.to_account_info(),anchor_spl::token_interface::TransferChecked{
   from:source.clone(),to:destination.clone(),authority:authority.clone(),mint:mint.clone(),
  },signer_seeds),amount,decimals,
 )
}
#[inline(never)]
pub fn collect_transfer_user_tokens_ref<'info>(
 token_program:&Interface<'info,anchor_spl::token_interface::TokenInterface>,
 source:&InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,
 destination:&InterfaceAccount<'info,anchor_spl::token_interface::TokenAccount>,
 authority:&&Signer<'info>,decimals:u8,mint:&AccountInfo<'info>,amount:u64,
)->Result<()>{
 anchor_spl::token_interface::transfer_checked(
  CpiContext::new(token_program.to_account_info(),anchor_spl::token_interface::TransferChecked{
   from:source.to_account_info(),to:destination.to_account_info(),authority:authority.to_account_info(),mint:mint.clone(),
  }),amount,decimals,
 )
}
/// Transfer curve tokens with the curve authority passed by double reference.
#[inline(never)]
pub fn transfer_curve_tokens_ref<'info>(
    token_program: &Interface<'info, TokenInterface>,
    source: &InterfaceAccount<'info, TokenAccount>,
    destination: &InterfaceAccount<'info, TokenAccount>,
    authority: &&mut Account<'info, BondingCurve>,
    decimals: u8,
    mint: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[&[u8]]; 1],
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: source.to_account_info(),
                to: destination.to_account_info(),
                authority: authority.to_account_info(),
                mint: mint.clone(),
            },
            signer_seeds,
        ),
        amount,
        decimals,
    )
}
#[inline(never)]
pub fn collect_transfer_user_quote_tokens<'info>(
 token_program:&Interface<'info,anchor_spl::token_interface::TokenInterface>,
 source:&UncheckedAccount<'info>,destination:&UncheckedAccount<'info>,authority:&AccountInfo<'info>,
 decimals:u8,mint:&AccountInfo<'info>,amount:u64,
)->Result<()>{
 if amount==0{return Ok(());}
 anchor_spl::token_interface::transfer_checked(
  CpiContext::new(token_program.to_account_info(),anchor_spl::token_interface::TransferChecked{
   from:source.to_account_info(),to:destination.to_account_info(),authority:authority.clone(),mint:mint.clone(),
  }),amount,decimals,
 )
}
#[inline(never)]
pub fn collect_transfer_vault_tokens<'info>(
 token_program:&Interface<'info,anchor_spl::token_interface::TokenInterface>,
 source:&UncheckedAccount<'info>,destination:&UncheckedAccount<'info>,authority:&AccountInfo<'info>,
 decimals:u8,mint:&AccountInfo<'info>,amount:u64,signer_seeds:&[&[&[u8]];1],
)->Result<()>{
 anchor_spl::token_interface::transfer_checked(
  CpiContext::new_with_signer(token_program.to_account_info(),anchor_spl::token_interface::TransferChecked{
   from:source.to_account_info(),to:destination.to_account_info(),authority:authority.clone(),mint:mint.clone(),
  },signer_seeds),amount,decimals,
 )
}
