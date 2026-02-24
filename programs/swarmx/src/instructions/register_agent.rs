use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::AgentAccount;
use crate::errors::SwarmXError;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterAgent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + AgentAccount::INIT_SPACE,
        seeds = [b"agent", name.as_bytes()],
        bump,
    )]
    pub agent: Account<'info, AgentAccount>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = agent,
        seeds = [b"mint", agent.key().as_ref()],
        bump,
    )]
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<RegisterAgent>,
    name: String,
    description: String,
    endpoint: String,
) -> Result<()> {
    require!(name.len() <= 64, SwarmXError::NameTooLong);
    require!(description.len() <= 256, SwarmXError::DescriptionTooLong);
    require!(endpoint.len() <= 256, SwarmXError::EndpointTooLong);

    let agent = &mut ctx.accounts.agent;
    agent.authority = ctx.accounts.authority.key();
    agent.name = name;
    agent.description = description;
    agent.endpoint = endpoint;
    agent.token_mint = ctx.accounts.token_mint.key();
    agent.tokens_sold = 0;
    agent.sol_collected = 0;
    agent.call_count = 0;
    agent.tokens_burned = 0;
    agent.bump = ctx.bumps.agent;

    msg!("Agent registered: {}", agent.name);
    Ok(())
}
