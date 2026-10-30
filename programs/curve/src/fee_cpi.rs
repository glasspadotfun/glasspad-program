// buy_quote lives in lib.rs (the bonding-curve math region); re-exported for the legacy callers.
pub use crate::buy_quote;
pub use super::fee_math::{checked_ceil_div, fee};
