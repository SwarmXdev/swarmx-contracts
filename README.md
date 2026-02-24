# 🔗 SwarmX Contracts

Solana smart contracts for the SwarmX AI Agent Token Marketplace.

## Overview

SwarmX enables AI agents to issue their own tokens via bonding curves, trade capabilities, and earn revenue on-chain.

## Architecture

```
programs/swarmx/src/
├── lib.rs                    # Program entrypoint
├── state.rs                  # Account structures (AgentAccount, PlatformConfig)
├── errors.rs                 # Custom error codes
└── instructions/
    ├── register_agent.rs     # Register agent + create token mint
    ├── buy_token.rs          # Buy tokens from bonding curve
    ├── sell_token.rs         # Sell tokens back to bonding curve
    └── call_agent.rs         # Service call — 40% burn / 40% dev / 20% platform
```

## Tokenomics

Each agent gets a unique SPL token with a linear bonding curve:

| Action | Distribution |
|--------|-------------|
| **Service Call** | 40% burned 🔥 / 40% to developer 👨‍💻 / 20% to platform 🏦 |
| **Buy** | SOL → bonding curve vault → mint tokens |
| **Sell** | Burn tokens → bonding curve vault → SOL |

## Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [Solana CLI](https://docs.solanalabs.com/cli/install) 1.18+
- [Anchor](https://www.anchor-lang.com/docs/installation) 0.30+

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy (devnet)

```bash
anchor deploy --provider.cluster devnet
```

## License

MIT
