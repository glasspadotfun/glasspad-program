//! the rules. there is no instruction that can change any of these.
use anchor_lang::prelude::*;

/// curve: the same numbers every degen already knows (6-decimal tokens).
pub const INITIAL_VIRTUAL_TOKEN_RESERVES: u64 = 1_073_000_000_000_000;
#[cfg(not(feature = "devnet"))]
pub const INITIAL_VIRTUAL_SOL_RESERVES: u64 = 30_000_000_000;
/// devnet test build only: a curve that fills with ~0.85 sol instead of ~85, same maths.
#[cfg(feature = "devnet")]
pub const INITIAL_VIRTUAL_SOL_RESERVES: u64 = 300_000_000;
pub const INITIAL_REAL_TOKEN_RESERVES: u64 = 793_100_000_000_000;
pub const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000_000_000;

/// fees, in basis points of every curve trade.
pub const PROTOCOL_FEE_BPS: u64 = 100;
pub const CREATOR_FEE_BPS: u64 = 40;
/// share of the protocol fee that goes to the burn pda (the rest goes to `TEAM`).
pub const BUYBACK_BPS: u64 = 5000;
/// taken from the curve's sol when a coin graduates, split like the protocol fee.
#[cfg(not(feature = "devnet"))]
pub const POOL_MIGRATION_FEE: u64 = 1_000_000_000;
#[cfg(feature = "devnet")]
pub const POOL_MIGRATION_FEE: u64 = 200_000_000;

/// the team's half of the protocol fee lands here. published on the docs page.
#[cfg(not(feature = "devnet"))]
pub const TEAM: Pubkey = anchor_lang::solana_program::pubkey!("5NmHtk2ajKQWGHFtb56BRgjGc248Z77ETUxmst4VERph");
/// devnet test build: the deployer wallet plays the team, so the canary can seal and crank.
#[cfg(feature = "devnet")]
pub const TEAM: Pubkey = anchor_lang::solana_program::pubkey!("6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy");
/// the burn half accrues in this pda until `burn_crank` swaps and burns it.
pub const BURN_SEED: &[u8] = b"burn";

pub fn burn_pda() -> Pubkey { Pubkey::find_program_address(&[BURN_SEED], &crate::ID).0 }
