use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::state::AgentAccount;
use crate::errors::SwarmXError;

const BASE_PRICE: u64 = 1_000;
const SLOPE: u64 = 10;

#[derive(Accounts)]
pub struct SellToken<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub agent: Account<'info, AgentAccount>,

    #[account(
        mut,
        constraint = token_mint.key() == agent.token_mint,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    /// CHECK: Bonding curve vault PDA
    #[account(
        mut,
        seeds = [b"vault", agent.key().as_ref()],
        bump,
    )]
    pub vault: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SellToken>, amount_token: u64) -> Result<()> {
    require!(amount_token > 0, SwarmXError::ZeroAmount);
    require!(
        ctx.accounts.seller_token_account.amount >= amount_token,
        SwarmXError::InsufficientTokens
    );

    let agent = &ctx.accounts.agent;
    let current_price = BASE_PRICE
        .checked_add(SLOPE.checked_mul(agent.tokens_sold).ok_or(SwarmXError::MathOverflow)?)
        .ok_or(SwarmXError::MathOverflow)?;

    // SOL returned = tokens * current_price / 10^9
    let sol_out = amount_token
        .checked_mul(current_price)
        .ok_or(SwarmXError::MathOverflow)?
        .checked_div(1_000_000_000)
        .ok_or(SwarmXError::MathOverflow)?;

    require!(sol_out > 0, SwarmXError::ZeroAmount);

    // Burn tokens
    let seeds = &[
        b"agent",
        ctx.accounts.agent.name.as_bytes(),
        &[ctx.accounts.agent.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.seller_token_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        amount_token,
    )?;

    // Transfer SOL from vault to seller
    let vault = &ctx.accounts.vault;
    **vault.to_account_info().try_borrow_mut_lamports()? -= sol_out;
    **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += sol_out;

    // Update agent state
    let agent = &mut ctx.accounts.agent;
    agent.tokens_sold = agent.tokens_sold
        .checked_sub(amount_token)
        .ok_or(SwarmXError::MathOverflow)?;
    agent.sol_collected = agent.sol_collected
        .checked_sub(sol_out)
        .ok_or(SwarmXError::MathOverflow)?;

    msg!("Sold {} tokens for {} lamports", amount_token, sol_out);
    Ok(())
}
