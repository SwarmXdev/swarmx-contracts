use anchor_lang::prelude::*;

#[error_code]
pub enum SwarmXError {
    #[msg("Agent name too long (max 64 chars)")]
    NameTooLong,
    #[msg("Agent description too long (max 256 chars)")]
    DescriptionTooLong,
    #[msg("Endpoint too long (max 256 chars)")]
    EndpointTooLong,
    #[msg("Insufficient SOL for purchase")]
    InsufficientSol,
    #[msg("Insufficient tokens for sale")]
    InsufficientTokens,
    #[msg("Insufficient tokens for call")]
    InsufficientCallTokens,
    #[msg("Bonding curve calculation overflow")]
    MathOverflow,
    #[msg("Zero amount not allowed")]
    ZeroAmount,
}
