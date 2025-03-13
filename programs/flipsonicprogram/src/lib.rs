use anchor_lang::prelude::*;

mod liquidity;
mod swap;
pub mod errors;

use crate::liquidity::*;
use crate::swap::*;

declare_id!("edJUEE32ixRxvoiCfVD9Svo5yaSrTNrgxrYDfEVyo1Q");

#[program]
pub mod flipsonicprogram {
    use super::*;

    //Create Liquidity
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        pool_bump: u8,
        fee: u16,
    ) -> Result<()> {
        liquidity::initialize_pool(ctx, pool_bump, fee)
    }

    // Add Liquidity
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a: u128,
        amount_b: u128,
        pool_bump: u8,
    ) -> Result<()> {
        liquidity::add_liquidity(ctx, amount_a, amount_b, pool_bump)
    }

    // Withdraw Liquidity
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        liquidity_tokens: u128,
        pool_bump: u8,
    ) -> Result<()> {
        liquidity::remove_liquidity(ctx, liquidity_tokens, pool_bump)
    }

    // Swap
    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u128,
        min_amount_out: u128,
        pool_bump: u8
    ) -> Result<()> {
        swap::swap(ctx, amount_in, min_amount_out, pool_bump)
    }
}

