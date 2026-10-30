use anchor_lang::prelude::*;

// layout-pad(8): aligns the next Source site with the original line







/// Shared checked ceiling division, preserving addition/subtraction/division order.
#[inline(never)]
pub fn checked_ceil_div(numerator: u128, denominator: u128) -> Result<u128> {
    let sum = numerator.checked_add(denominator)
        .ok_or_else(|| error!(crate::PumpError::Overflow))?;
    let adjusted = sum.checked_sub(1)
        .ok_or_else(|| error!(crate::PumpError::Overflow))?;
    adjusted.checked_div(denominator)
        .ok_or_else(|| error!(crate::PumpError::DivisionByZero))
}

pub fn fee(quote_amount: u64, basis_points: u64) -> Result<u64> {
    let value = checked_ceil_div(u128::from(quote_amount) * u128::from(basis_points), 10000)?;
// layout-pad(10): aligns the next Source site with the original line









    u64::try_from(value).map_err(|_| error!(crate::PumpError::Truncation))
}

use crate::trade_types::BondingCurve;

/// the only fee rule: two constants, creator fee only when the coin has a creator.
pub fn fees_for(curve: &BondingCurve, quote_amount: u64) -> Result<(u64, u64, u64, u64)> {
    let protocol_fee = fee(quote_amount, crate::consts::PROTOCOL_FEE_BPS)?;
    let creator_fee = if curve.creator != Pubkey::default() { fee(quote_amount, crate::consts::CREATOR_FEE_BPS)? } else { 0 };
    Ok((protocol_fee, creator_fee, crate::consts::PROTOCOL_FEE_BPS, crate::consts::CREATOR_FEE_BPS))
}
