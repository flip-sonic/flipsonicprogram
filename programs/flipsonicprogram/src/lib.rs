use anchor_lang::prelude::*;

pub mod liquidity;
pub mod swap;
pub mod errors;

use crate::liquidity::*;
use crate::swap::*;

pub mod state;
pub use state::*;

declare_id!("5MnVMGkWMYz5NUnrt9So1qwHQRauFa7aqhMfj83mAMBU");


#[program]
pub mod flipsonicprogram {
    use super::*;

    // Swap

    // Swap Quote

    //Create Liquidity

    // Add Liquidity

    // Withdraw Liquidity
    
    // Initialize the program
    pub fn create_pool(ctx: Context<CreatePool>, bump: u8, initial_a: u64, initial_b: u64) -> Result<()> {
        create::create_pool(ctx, bump, initial_a, initial_b)
    }
}