use crate::contants::STAKE_INFO_SEED;
use crate::errors::AppError;
use crate::state::StakeInfo;
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    pub mint: Account<'info, Mint>, // Token được stake

    #[account(
        init_if_needed,
        payer = staker,
        seeds = [STAKE_INFO_SEED, staker.key().as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + StakeInfo::INIT_SPACE
    )]
    pub stake_info: Account<'info, StakeInfo>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = mint,
        associated_token::authority = stake_info,
    )]
    pub vault_token_account: Account<'info, TokenAccount>, // Token được giữ trong PDA

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = staker,
    )]
    pub staker_token_account: Account<'info, TokenAccount>, // Token từ người dùng

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let stake_info = &mut ctx.accounts.stake_info;

    require!(amount > 0, AppError::NoToken);

    let clock = Clock::get()?;

    stake_info.staker = ctx.accounts.staker.key();
    stake_info.mint = ctx.accounts.mint.key();
    stake_info.stake_at = clock.slot;
    stake_info.is_staked = true;   
    // Cộng dồn số lượng stake
    stake_info.amount = stake_info.amount.checked_add(amount).unwrap();

    // Chuyển token vào vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staker_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.staker.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
