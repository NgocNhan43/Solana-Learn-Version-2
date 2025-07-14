use crate::contants::REWARD_VAULT_SEED;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint: Account<'info, Mint>, // Token dùng làm phần thưởng

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [REWARD_VAULT_SEED, mint.key().as_ref()], // Vault riêng cho mỗi token
        bump,
        token::mint = mint,
        token::authority = reward_vault,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

// Không cần logic gì, chỉ tạo vault nếu chưa tồn tại
pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
