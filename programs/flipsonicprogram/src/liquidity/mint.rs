use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, mint_to, Burn, Mint, MintTo,
        Token, TokenAccount,
    },
};

use super::Pool;

// Mint tokens to a user
pub fn mint_to_user<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, MintToUser<'info>>,
    amount: u64,
    bump: u8,
) -> Result<()> {
    let pool = ctx.accounts.pool;

    let mint_a = pool.mint_a;
    let mint_b = pool.mint_b;
    let owner = pool.owner;
    
    let seeds = &[b"pool".as_ref(), mint_a.as_ref(), mint_b.as_ref(), owner.as_ref(), &[bump]];
  
    let signer = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: pool.to_account_info(),
            },
            signer,
        ),
        amount,
    )?;

    msg!("Tokens minted successfully by Psolite program.");

    Ok(())
}

pub fn burn_user_token<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, BurnUserToken<'info>>,
    amount: u64,
) -> Result<()> {
    // Burn tokens from the user's token account
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(), // Must match the token owner
            },
        ),
        amount,
    )?;

    msg!("Tokens burned successfully by Psolite Program.");

    Ok(())
}

#[derive(Accounts)]
pub struct BurnUserToken<'info> {
    /// CHECK: Safe because we check seeds
    #[account(seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref(), pool.owner.as_ref()], bump)]
    pub pool: Account<'info, Pool>,

    #[account(mut, constraint = mint.key() == pool.liquidity_token_mint)] 
    pub mint: Account<'info, Mint>, // Token mint

    #[account(
        mut, 
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == mint.key()
    )] 
    pub user_token_account: Account<'info, TokenAccount>, // User's token account

    #[account(mut)]
    pub user: Signer<'info>, // The user burning their tokens

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MintToUser<'info> {
    /// CHECK: Safe because we check seeds
    #[account(seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref(), pool.owner.as_ref()], bump)]
    pub pool: Account<'info, Pool>,

    #[account(mut, constraint = mint.key() == pool.liquidity_token_mint)]
    pub mint: Account<'info, Mint>,

    #[account(mut,
        constraint = recipient_token_account.owner == user.key(),
        constraint = recipient_token_account.mint == mint.key()
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

