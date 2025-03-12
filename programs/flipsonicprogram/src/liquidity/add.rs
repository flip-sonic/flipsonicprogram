use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use num_integer::Roots;

use crate::liquidity::create::Pool;
use crate::liquidity::mint::{mint_to_user, MintToUser};
use crate::liquidity::transfer::{transfer_from_user, TransferFromUser};

// Add liquidity to the pool and mint liquidity tokens
pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    amount_a: u64,
    amount_b: u64,
    pool_bump: u8,
) -> Result<()> {
    // Transfer tokens from user to pool
    transfer_from_user(ctx.accounts.transfer_a_context(), amount_a)?;
    transfer_from_user(ctx.accounts.transfer_b_context(), amount_b)?;
 

    // Calculate liquidity tokens to mint
    let total_liquidity = ctx.accounts.pool.total_liquidity;
    let liquidity_tokens = if total_liquidity == 0 {
        // Initial liquidity
        amount_a.checked_mul(amount_b).unwrap().sqrt()
    } else {
        // Proportional liquidity
        let liquidity_a = amount_a
            .checked_mul(total_liquidity)
            .unwrap()
            .checked_div(ctx.accounts.pool.reserve_a)
            .unwrap();
        let liquidity_b = amount_b
            .checked_mul(total_liquidity)
            .unwrap()
            .checked_div(ctx.accounts.pool.reserve_b)
            .unwrap();
        liquidity_a.min(liquidity_b)
    };
     
    // Mint liquidity tokens to the user
    mint_to_user(
        ctx.accounts.mint_liquidity_tokens_context(),
        liquidity_tokens,
        pool_bump,
    )?;

    // Update pool reserves and total liquidity
    let pool = &mut ctx.accounts.pool;

    pool.reserve_a += amount_a;
    pool.reserve_b += amount_b;
    pool.total_liquidity += liquidity_tokens;

    Ok(())
}

impl<'info> AddLiquidity<'info> {
    fn transfer_a_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferFromUser<'info>> {
        let cpi_accounts = TransferFromUser {
            source: self.user_token_a.clone(),
            destination: self.pool_token_a.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_b_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferFromUser<'info>> {
        let cpi_accounts = TransferFromUser {
            source: self.user_token_b.clone(),
            destination: self.pool_token_b.clone(),
            user: self.user.clone(),
            token_program: self.token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn mint_liquidity_tokens_context(&self) -> CpiContext<'_, '_, '_, 'info, MintToUser<'info>> {
        let cpi_accounts = MintToUser {
            pool: self.pool.clone(),
            mint: self.liquidity_token_mint.clone(),
            user: self.user.clone(),
            recipient_token_account: self.user_liquidity_token_account.clone(),
            token_program: self.token_program.clone(),
            system_program: self.system_program.clone(),
            rent: self.rent.clone(),
            associated_token_program: self.associated_token_program.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

// Accounts for adding liquidity
#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut, constraint = liquidity_token_mint.key() == pool.liquidity_token_mint)]
    pub liquidity_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_liquidity_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_token: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
