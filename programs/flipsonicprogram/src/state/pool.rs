use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool  {
    pub id: u64,
    pub authority: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub lp_mint: Pubkey,
    pub bump: u8,
}