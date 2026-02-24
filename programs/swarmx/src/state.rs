use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AgentAccount {
    /// Agent owner (developer wallet)
    pub authority: Pubkey,
    /// Agent name
    #[max_len(64)]
    pub name: String,
    /// Agent description
    #[max_len(256)]
    pub description: String,
    /// API endpoint for calling this agent
    #[max_len(256)]
    pub endpoint: String,
    /// Token mint for this agent
    pub token_mint: Pubkey,
    /// Total tokens sold via bonding curve
    pub tokens_sold: u64,
    /// Total SOL collected in bonding curve
    pub sol_collected: u64,
    /// Total times this agent has been called
    pub call_count: u64,
    /// Total tokens burned from calls
    pub tokens_burned: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    /// Platform admin
    pub admin: Pubkey,
    /// Platform fee wallet (receives 20% of call fees)
    pub fee_wallet: Pubkey,
    /// Total agents registered
    pub agent_count: u64,
    /// Bump seed
    pub bump: u8,
}
