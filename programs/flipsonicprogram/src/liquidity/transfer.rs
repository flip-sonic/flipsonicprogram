use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use super::Pool;

pub fn transfer_from_user<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, TransferFromUser<'info>>,
    amount: u64,
) -> Result<()> {

    // Transfer the tokens from the user to the staking contract
    let cpi_accounts = Transfer {
        from: ctx.accounts.source.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    Ok(())
}

pub fn transfer_to_user<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, TransferToUser<'info>>,
    amount: u64,
    bump: u8,
) -> Result<()> {

    let pool = &ctx.accounts.pool;

    let mint_a = pool.mint_a;
    let mint_b = pool.mint_b;
    let owner = pool.owner;
    
    let seeds = &[b"pool".as_ref(), mint_a.as_ref(), mint_b.as_ref(), owner.as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.source.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferFromUser<'info> {
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferToUser<'info> {

    #[account(mut,
        token::authority = pool,
        token::token_program = token_program,
    )]
    pub source: Account<'info, TokenAccount>,

    #[account(mut,
        token::authority = user,
        token::token_program = token_program,
    )]
    pub destination: Account<'info, TokenAccount>,

    /// CHECK: Safe because we check seeds
    #[account(seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref(), pool.owner.as_ref()], bump)]
    pub pool: Account<'info, Pool>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}


