use crate::contants::{REWARD_VAULT_SEED, STAKE_INFO_SEED};
use crate::errors::AppError;
use crate::state::StakeInfo;
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [STAKE_INFO_SEED, staker.key().as_ref(), mint.key().as_ref()],
        bump,
        has_one = staker,
        has_one = mint,
        close = staker // sau khi unstake hết thì đóng account, hoàn trả lamport
    )]
    pub stake_info: Account<'info, StakeInfo>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = stake_info,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [REWARD_VAULT_SEED, mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = reward_vault,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = staker,
    )]
    pub staker_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let stake_info = &ctx.accounts.stake_info;

    // Kiểm tra số lượng rút phải > 0 và <= số lượng đang stake
    require!(amount > 0 && amount <= stake_info.amount, AppError::NoToken);

    // Tính số block đã stake
    let clock = Clock::get()?;
    let slot_passed = clock.slot - stake_info.stake_at;

    // Tính phần thưởng: 1% mỗi block trên số lượng unstake
    let reward = amount
        .checked_mul(slot_passed)
        .unwrap()
        .checked_div(100)
        .unwrap();

    msg!("Unstaking {} tokens after {} slots => reward: {}", amount, slot_passed, reward);

    // ==== Chuyển reward ====
    let mint_key = ctx.accounts.mint.key(); // cần tách ra để tránh lỗi borrow tạm thời
    let reward_vault_signer_seeds: &[&[&[u8]]] = &[&[
        REWARD_VAULT_SEED,
        mint_key.as_ref(),
        &[ctx.bumps.reward_vault],
    ]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reward_vault.to_account_info(),
                to: ctx.accounts.staker_token_account.to_account_info(),
                authority: ctx.accounts.reward_vault.to_account_info(),
            },
            reward_vault_signer_seeds,
        ),
        reward,
    )?;

    // ==== Chuyển lại token đã stake ====
    let staker_key = ctx.accounts.staker.key(); // tránh giá trị tạm thời
    let stake_info_signer_seeds: &[&[&[u8]]] = &[&[
        STAKE_INFO_SEED,
        staker_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.stake_info],
    ]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.staker_token_account.to_account_info(),
                authority: ctx.accounts.stake_info.to_account_info(),
            },
            stake_info_signer_seeds,
        ),
        amount,
    )?;

    // ==== Cập nhật hoặc đóng stake_info ====
    let stake_info = &mut ctx.accounts.stake_info;

    if stake_info.amount == amount {
        // sẽ bị đóng tự động do close = staker
        msg!("Unstaked toàn bộ => đóng stake_info");
    } else {
        // chỉ cập nhật nếu vẫn còn token
        stake_info.amount -= amount;
        stake_info.stake_at = clock.slot; // cập nhật lại thời gian stake cho phần còn lại
    }

    Ok(())
}
