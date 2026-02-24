use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("Duf2CX5ZGUgLddKffADWc6RygEEYawmqZxzpKGHzfMVE");

#[program]
pub mod swarmx {
    use super::*;

    /// Register a new agent and create its token via bonding curve
    pub fn register_agent(
        ctx: Context<RegisterAgent>,
        name: String,
        description: String,
        endpoint: String,
    ) -> Result<()> {
        instructions::register_agent::handler(ctx, name, description, endpoint)
    }

    /// Buy agent tokens from the bonding curve
    pub fn buy_token(ctx: Context<BuyToken>, amount_sol: u64) -> Result<()> {
        instructions::buy_token::handler(ctx, amount_sol)
    }

    /// Sell agent tokens back to the bonding curve
    pub fn sell_token(ctx: Context<SellToken>, amount_token: u64) -> Result<()> {
        instructions::sell_token::handler(ctx, amount_token)
    }

    /// Record a service call — burns 40%, sends 40% to dev, 20% to platform
    pub fn call_agent(ctx: Context<CallAgent>, token_amount: u64) -> Result<()> {
        instructions::call_agent::handler(ctx, token_amount)
    }
}
