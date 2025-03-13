use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::AmmError;
use crate::liquidity::create::Pool;
use crate::liquidity::transfer::{transfer_from_user,transfer_to_user, TransferFromUser, TransferToUser};

// Swap tokens
pub fn swap(
    ctx: Context<Swap>,
    amount_in: u128,
    min_amount_out: u128,
    pool_bump: u8
) -> Result<()> {

    require!(ctx.accounts.user_token_in.mint == ctx.accounts.pool.mint_a || ctx.accounts.user_token_in.mint == ctx.accounts.pool.mint_b , AmmError::WrongPool);
    require!(ctx.accounts.user_token_out.mint == ctx.accounts.pool.mint_a || ctx.accounts.user_token_out.mint == ctx.accounts.pool.mint_b, AmmError::WrongPool);

    // Deduct the fee (e.g., 0.3%)
    let fee = amount_in
        .checked_mul(ctx.accounts.pool.fee as u128)
        .ok_or(AmmError::MathError)?
        .checked_div(10000)
        .ok_or(AmmError::MathError)?;
    let amount_in_after_fee = amount_in.checked_sub(fee).ok_or(AmmError::MathError)?;
    
    // Calculate the output amount
    let amount_out = if ctx.accounts.user_token_in.mint == ctx.accounts.pool.mint_a {
        // Swap Token A to Token B
        (ctx.accounts.pool.reserve_b)
            .checked_mul(amount_in_after_fee as u128)
            .ok_or(AmmError::MathError)?
            .checked_div((ctx.accounts.pool.reserve_a) + (amount_in_after_fee as u128))
            .ok_or(AmmError::MathError)?
    } else {
        // Swap Token B to Token A
        (ctx.accounts.pool.reserve_a)
            .checked_mul(amount_in_after_fee as u128)
            .ok_or(AmmError::MathError)?
            .checked_div((ctx.accounts.pool.reserve_b) + (amount_in_after_fee as u128))
            .ok_or(AmmError::MathError)?
    } ;
    
    require!(amount_out >= min_amount_out, AmmError::SlippageExceeded);
 
    // Transfer tokens
    transfer_from_user(ctx.accounts.transfer_in_context(), amount_in as u64)?;
    transfer_to_user(ctx.accounts.transfer_out_context(), amount_out as u64, pool_bump)?;

    let pool = &mut ctx.accounts.pool;

    // Update reserves
    if ctx.accounts.user_token_in.mint == pool.mint_a {
        // Swap Token A to Token B
        pool.reserve_a += amount_in;
        pool.reserve_b -= amount_out;
    } else {
        // Swap Token B to Token A
        pool.reserve_b += amount_in;
        pool.reserve_a -= amount_out;
    }

    Ok(())
}

impl<'info> Swap<'info> {
    fn transfer_in_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferFromUser<'info>> {
        let cpi_accounts = TransferFromUser {
            source: self.user_token_in.clone(),
            destination: self.pool_token_in.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_out_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferToUser<'info>> {
        let cpi_accounts = TransferToUser {
            source: self.pool_token_out.clone(),
            destination: self.user_token_out.clone(),
            pool: self.pool.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

// Accounts for swapping
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_out: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}