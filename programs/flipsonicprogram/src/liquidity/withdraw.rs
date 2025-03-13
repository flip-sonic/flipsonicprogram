use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::liquidity::create::Pool;
use crate::liquidity::mint::{burn_user_token, BurnUserToken};
use crate::liquidity::transfer::{transfer_to_user, TransferToUser};

// Remove liquidity from the pool and burn liquidity tokens
pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    liquidity_tokens:  u128,
    pool_bump: u8,
) -> Result<()> {

    // Calculate amounts to withdraw
    let amount_a = liquidity_tokens.checked_mul(ctx.accounts.pool.reserve_a).unwrap().checked_div(ctx.accounts.pool.total_liquidity).unwrap();
    let amount_b = liquidity_tokens.checked_mul(ctx.accounts.pool.reserve_b).unwrap().checked_div(ctx.accounts.pool.total_liquidity).unwrap();

    // Burn liquidity tokens
    burn_user_token(ctx.accounts.burn_liquidity_tokens_context(), liquidity_tokens as u64)?;

    // Transfer tokens back to the user
    transfer_to_user(ctx.accounts.transfer_a_context(), amount_a as u64, pool_bump)?;
    transfer_to_user(ctx.accounts.transfer_b_context(), amount_b as u64, pool_bump)?;


    let pool = &mut ctx.accounts.pool;
    // Update pool reserves and total liquidity
    pool.reserve_a -= amount_a;
    pool.reserve_b -= amount_b;
    pool.total_liquidity -= liquidity_tokens;

    Ok(())
}

impl<'info> RemoveLiquidity<'info> {
    fn transfer_a_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferToUser<'info>> {
        let cpi_accounts = TransferToUser {
            source: self.pool_token_a.clone(),
            destination: self.user_token_a.clone(),
            pool: self.pool.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_b_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferToUser<'info>> {
        let cpi_accounts = TransferToUser {
            source: self.pool_token_b.clone(),
            destination: self.user_token_b.clone(),
            pool: self.pool.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn burn_liquidity_tokens_context(&self) -> CpiContext<'_, '_, '_, 'info, BurnUserToken<'info>> {
        let cpi_accounts = BurnUserToken {
            pool: self.pool.clone(),
            mint: self.liquidity_token_mint.clone(),
            user: self.user.clone(),
            user_token_account: self.user_liquidity_token_account.clone(),
            token_program: self.token_program.clone()
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}


// Accounts for removing liquidity
#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut, constraint = pool.liquidity_token_mint == liquidity_token_mint.key())]
    pub liquidity_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}