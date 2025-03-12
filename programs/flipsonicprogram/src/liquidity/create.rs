use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

// Initialize a new liquidity pool
pub fn initialize_pool(ctx: Context<InitializePool>, pool_bump: u8, fee: u16) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.mint_a = ctx.accounts.mint_a.key();
    pool.mint_b = ctx.accounts.mint_b.key();
    pool.owner = ctx.accounts.user.key();
    pool.fee = fee;
    pool.bump = pool_bump;
    pool.liquidity_token_mint = ctx.accounts.liquidity_token_mint.key();
    Ok(())
}

// Accounts for initializing a pool
#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<Pool>(), seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref(), user.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        seeds = [b"pool", pool.key().as_ref()],
        bump,
        mint::authority = pool,
        mint::decimals = 6
    )]
    pub liquidity_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

// Pool account structure
#[account]
pub struct Pool {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub owner: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee: u16,
    pub bump: u8,
    pub liquidity_token_mint: Pubkey,
    pub total_liquidity: u64,
}
