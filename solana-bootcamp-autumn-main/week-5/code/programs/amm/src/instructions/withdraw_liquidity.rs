use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, Token, TokenAccount, Transfer},
};
use spl_math::uint::U256;

use crate::state::Pool;

pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, anchor_lang::error::ErrorCode::InvalidArgument);

    let lp_supply = ctx.accounts.mint_liquidity.supply;
    require!(lp_supply > 0, anchor_lang::error::ErrorCode::InvalidArgument);

    let pool_a_amount = ctx.accounts.pool_account_a.amount;
    let pool_b_amount = ctx.accounts.pool_account_b.amount;

    // Calculate amount to return based on share
    let amount_a = U256::from(amount)
        .checked_mul(U256::from(pool_a_amount))
        .unwrap()
        .checked_div(U256::from(lp_supply))
        .unwrap()
        .as_u64();

    let amount_b = U256::from(amount)
        .checked_mul(U256::from(pool_b_amount))
        .unwrap()
        .checked_div(U256::from(lp_supply))
        .unwrap()
        .as_u64();

    // Derive PDA signer
    let authority_bump = ctx.bumps.pool_authority;
    let authority_seeds = &[
        ctx.accounts.pool.amm.as_ref(),
        ctx.accounts.pool.mint_a.as_ref(),
        ctx.accounts.pool.mint_b.as_ref(),
        b"authority",
        &[authority_bump],
    ];
    let signer = &[&authority_seeds[..]];

    // Transfer A from pool to user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_account_a.to_account_info(),
                to: ctx.accounts.depositor_account_a.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            signer,
        ),
        amount_a,
    )?;

    // Transfer B from pool to user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_account_b.to_account_info(),
                to: ctx.accounts.depositor_account_b.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            signer,
        ),
        amount_b,
    )?;

    // Burn LP token
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_liquidity.to_account_info(),
                from: ctx.accounts.depositor_account_liquidity.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(
        mut,
        seeds = [
            pool.amm.key().as_ref(),
            pool.mint_a.key().as_ref(),
            pool.mint_b.key().as_ref()
        ],
        bump,
        has_one = mint_a,
        has_one = mint_b
    )]
    pub pool: Box<Account<'info, Pool>>,

    /// CHECK only used for PDA
    #[account(
        seeds = [
            pool.amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            b"authority"
        ],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    pub mint_a: Box<Account<'info, Mint>>,
    pub mint_b: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            pool.amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            b"mint_liquidity"
        ],
        bump
    )]
    pub mint_liquidity: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
