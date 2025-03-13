use anchor_lang::prelude::*;

// Custom errors
#[error_code]
pub enum AmmError {
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Invalid Token Amount")]
    InvalidTokenAmount,
    #[msg("In Balance amount")]
    InvalidRatio,
}