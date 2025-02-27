use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked,  MintTo, Burn, transfer_checked, mint_to, burn, },
};

use crate::Pool;
use num_integer::Roots;


    pub fn create_pool(
        ctx: Context<CreatePool>,
        bump: u8,
        initial_a: u64,
        initial_b: u64,
    ) -> Result<()> {
        // Create the pool
        let pool= &mut ctx.accounts.pool;

        pool.token_a_mint = ctx.accounts.token_a_mint.key();
        pool.token_b_mint = ctx.accounts.token_b_mint.key();
        pool.token_a_account = ctx.accounts.token_a_account.key();
        pool.token_b_account = ctx.accounts.token_b_account.key();
        pool.lp_mint = ctx.accounts.lp_mint.key(); 
        pool.bump = bump;

        // Transfer initial liquidity from user to pool
        let decimals_a = ctx.accounts.token_a_mint.decimals;
        let cpi_accounts_a = TransferChecked {
            from: ctx.accounts.user_token_a.to_account_info(),
            mint: ctx.accounts.token_a_mint.to_account_info(),
            to: ctx.accounts.token_a_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program.clone(), cpi_accounts_a);
        transfer_checked(cpi_context, initial_a, decimals_a)?;

        let decimals_b = ctx.accounts.token_b_mint.decimals;
        let cpi_accounts_b = TransferChecked {
            from: ctx.accounts.user_token_b.to_account_info(),
            mint: ctx.accounts.token_b_mint.to_account_info(),
            to: ctx.accounts.token_b_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts_b);
        transfer_checked(cpi_context, initial_b, decimals_b)?;


        // Mint initial LP tokens (sqrt(initial_a * initial_b))
        let lp_amount = (initial_a as u128 * initial_b as u128).sqrt().try_into().unwrap();

        let decimals_lp = ctx.accounts.lp_mint.decimals;
        let seeds = &[pool.to_account_info().key.as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.lp_mint.to_account_info(),
            to: ctx.accounts.user_lp_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            ),
            lp_amount,
        )?;



        Ok(())
    }

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space =  8 + 32 * 5 + 1, // Discriminator + 5 Pubkeys + bump
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        seeds = [pool.key().as_ref()],
        bump
    )]
    /// CHECK: PDA authority, validated by seeds
    pub authority: AccountInfo<'info>,
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_a_account: InterfaceAccount<'info, TokenAccount>, // Pool's Token A account
    #[account(mut)]
    pub token_b_account: InterfaceAccount<'info, TokenAccount>, // Pool's Token B account

    #[account(mut)]
    pub lp_mint: InterfaceAccount<'info, Mint>, // Pool's LP token mint

    #[account(mut)]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_account: InterfaceAccount<'info, TokenAccount>,


    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}