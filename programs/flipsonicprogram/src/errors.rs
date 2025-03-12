use anchor_lang::prelude::*;

// Custom errors
#[error_code]
pub enum AmmError {
    #[msg("Slippage exceeded")]
    SlippageExceeded,
}