use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::AmmError;
use crate::liquidity::create::Pool;
use crate::liquidity::transfer::{transfer_from_user,transfer_to_user, TransferFromUser, TransferToUser};

// Swap tokens
pub fn swap(
    ctx: Context<Swap>,
    amount_in: u64,
    min_amount_out: u64,
    pool_bump: u8
) -> Result<()> {

    // Calculate output amount using constant product formula
    let amount_out = (ctx.accounts.pool.reserve_b * amount_in) / (ctx.accounts.pool.reserve_a + amount_in);
    require!(amount_out >= min_amount_out, AmmError::SlippageExceeded);

    // Transfer tokens
    transfer_from_user(ctx.accounts.transfer_in_context(), amount_in)?;
    transfer_to_user(ctx.accounts.transfer_out_context(), amount_out, pool_bump)?;

    let pool = &mut ctx.accounts.pool;

    // Update reserves
    pool.reserve_a += amount_in;
    pool.reserve_b -= amount_out;

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