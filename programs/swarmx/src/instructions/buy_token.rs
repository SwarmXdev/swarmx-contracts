use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::state::AgentAccount;
use crate::errors::SwarmXError;

/// Simple linear bonding curve: price = base_price + slope * tokens_sold
/// base_price = 1000 lamports, slope = 10 lamports per token
const BASE_PRICE: u64 = 1_000;
const SLOPE: u64 = 10;

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub agent: Account<'info, AgentAccount>,

    #[account(
        mut,
        constraint = token_mint.key() == agent.token_mint,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    /// CHECK: Bonding curve vault PDA that holds SOL
    #[account(
        mut,
        seeds = [b"vault", agent.key().as_ref()],
        bump,
    )]
    pub vault: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyToken>, amount_sol: u64) -> Result<()> {
    require!(amount_sol > 0, SwarmXError::ZeroAmount);

    let agent = &ctx.accounts.agent;
    let current_sold = agent.tokens_sold;

    // Calculate how many tokens the buyer gets for amount_sol
    // Integral of (base_price + slope * x) dx from current_sold to current_sold + tokens_out
    // Simplified: tokens_out ≈ amount_sol / current_price
    let current_price = BASE_PRICE
        .checked_add(SLOPE.checked_mul(current_sold).ok_or(SwarmXError::MathOverflow)?)
        .ok_or(SwarmXError::MathOverflow)?;

    let tokens_out = amount_sol
        .checked_mul(1_000_000_000) // 9 decimals
        .ok_or(SwarmXError::MathOverflow)?
        .checked_div(current_price)
        .ok_or(SwarmXError::MathOverflow)?;

    require!(tokens_out > 0, SwarmXError::ZeroAmount);

    // Transfer SOL from buyer to vault
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        amount_sol,
    )?;

    // Mint tokens to buyer
    let agent_key = ctx.accounts.agent.key();
    let seeds = &[
        b"agent",
        ctx.accounts.agent.name.as_bytes(),
        &[ctx.accounts.agent.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.agent.to_account_info(),
            },
            signer_seeds,
        ),
        tokens_out,
    )?;

    // Update agent state
    let agent = &mut ctx.accounts.agent;
    agent.tokens_sold = agent.tokens_sold
        .checked_add(tokens_out)
        .ok_or(SwarmXError::MathOverflow)?;
    agent.sol_collected = agent.sol_collected
        .checked_add(amount_sol)
        .ok_or(SwarmXError::MathOverflow)?;

    msg!("Bought {} tokens for {} lamports", tokens_out, amount_sol);
    Ok(())
}
