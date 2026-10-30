use anchor_lang::prelude::*;
use crate::trade_types::BondingCurve;

#[inline(never)]
pub fn transfer_native<'info,T:AsRef<AccountInfo<'info>>>(
    source:&Account<'info,BondingCurve>,destination:T,amount:u64,
)->Result<()>{
    if amount==0{return Ok(())}
    source.sub_lamports(amount)?;
    destination.add_lamports(amount)?;
    Ok(())
}

/// Lamport move without the zero-amount early return (callers check the amount).
#[inline(never)]
pub fn move_native<'a,'info,T:AsRef<AccountInfo<'info>>>(
    source:&Account<'a,BondingCurve>,destination:T,amount:u64,
)->Result<()>{
    source.sub_lamports(amount)?;
    destination.add_lamports(amount)?;
    Ok(())
}

/// Lamport payout from the curve to a user-side account (zero amounts are a no-op).
#[inline(never)]
pub fn pay_out_native_owned<'info>(source:&Account<'info,BondingCurve>,destination:AccountInfo<'info>,amount:u64)->Result<()>{if amount==0{return Ok(())} source.sub_lamports(amount)?; destination.add_lamports(amount)?; Ok(())}
#[inline(never)]
pub fn pay_out_native<'info,T:AsRef<AccountInfo<'info>>>(source:&Account<'info,BondingCurve>,destination:T,amount:u64)->Result<()>{
    if amount==0{return Ok(())}
    source.sub_lamports(amount)?;
    destination.add_lamports(amount)?;
    Ok(())
}




#[inline(never)]
pub fn transfer_native_unchecked<'info,T:AsRef<AccountInfo<'info>>>(
    source:&Account<'info,BondingCurve>,destination:T,amount:u64,
)->Result<()>{
    source.sub_lamports(amount)?;
    destination.add_lamports(amount)?;
    Ok(())
}
