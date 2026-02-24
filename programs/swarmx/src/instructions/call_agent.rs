use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Transfer, Mint, Token, TokenAccount};

use crate::state::AgentAccount;
use crate::errors::SwarmXError;

/// 40% burn, 40% developer, 20% platform
const BURN_BPS: u64 = 4000;
const DEV_BPS: u64 = 4000;
const PLATFORM_BPS: u64 = 2000;

#[derive(Accounts)]
pub struct CallAgent<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(mut)]
    pub agent: Account<'info, AgentAccount>,

    #[account(
        mut,
        constraint = token_mint.key() == agent.token_mint,
    )]
    pub token_mint: Account<'info, Mint>,

    /// Caller's token account
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = caller,
    )]
    pub caller_token_account: Account<'info, TokenAccount>,

    /// Developer's token account (receives 40%)
    #[account(
        mut,
        token::mint = token_mint,
    )]
    pub dev_token_account: Account<'info, TokenAccount>,

    /// Platform's token account (receives 20%)
    #[account(
        mut,
        token::mint = token_mint,
    )]
    pub platform_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CallAgent>, token_amount: u64) -> Result<()> {
    require!(token_amount > 0, SwarmXError::ZeroAmount);
    require!(
        ctx.accounts.caller_token_account.amount >= token_amount,
        SwarmXError::InsufficientCallTokens
    );

    let burn_amount = token_amount
        .checked_mul(BURN_BPS)
        .ok_or(SwarmXError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(SwarmXError::MathOverflow)?;

    let dev_amount = token_amount
        .checked_mul(DEV_BPS)
        .ok_or(SwarmXError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(SwarmXError::MathOverflow)?;

    let platform_amount = token_amount
        .checked_sub(burn_amount)
        .ok_or(SwarmXError::MathOverflow)?
        .checked_sub(dev_amount)
        .ok_or(SwarmXError::MathOverflow)?;

    // Burn 40%
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.caller_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            },
        ),
        burn_amount,
    )?;

    // Transfer 40% to developer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.caller_token_account.to_account_info(),
                to: ctx.accounts.dev_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            },
        ),
        dev_amount,
    )?;

    // Transfer 20% to platform
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.caller_token_account.to_account_info(),
                to: ctx.accounts.platform_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            },
        ),
        platform_amount,
    )?;

    // Update agent state
    let agent = &mut ctx.accounts.agent;
    agent.call_count = agent.call_count
        .checked_add(1)
        .ok_or(SwarmXError::MathOverflow)?;
    agent.tokens_burned = agent.tokens_burned
        .checked_add(burn_amount)
        .ok_or(SwarmXError::MathOverflow)?;

    msg!(
        "Agent {} called. Burned: {}, Dev: {}, Platform: {}",
        agent.name, burn_amount, dev_amount, platform_amount
    );
    Ok(())
}
