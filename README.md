# 🌑 Void SDK

**The Confidential DeFi Layer for Solana** — Powered by [Arcium](https://arcium.com) MPC

[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?style=flat-square&logo=solana)](https://solana.com)
[![Arcium](https://img.shields.io/badge/Arcium-MPC-00D4AA?style=flat-square)](https://arcium.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

> **Privacy Hackathon 2026 Submission**  
> 🎯 Arcium Prize Track • 🔧 Privacy Tooling Track

---

## 📖 Overview

Void SDK enables **private DeFi operations** on Solana using Arcium's Multi-Party Computation (MPC) network. Unlike traditional solutions that only hide transfer amounts, Void SDK provides:

- 🔐 **Encrypted Account State** — Balances stored as ciphertext on-chain
- 🌊 **Confidential Swaps** — Trade without revealing amounts or slippage
- 👻 **Private Agent Actions** — AI agents can execute strategies without leaking alpha

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    TypeScript SDK                           │
│                  @yurei/void-sdk                            │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                 Anchor Program                              │
│               void_protocol.so                              │
│         (queue_computation, callbacks)                      │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│              Arcium MPC Network                             │
│          encrypted-ixs (MPC Circuits)                       │
│    PrivateAccount { owner, balance, token_mint }            │
└─────────────────────────────────────────────────────────────┘
```

## 📂 Project Structure

```
void-sdk/
├── anchor/                    # Solana programs
│   ├── encrypted-ixs/         # MPC circuit definitions
│   │   └── src/lib.rs         # PrivateAccount struct + init_account
│   ├── programs/
│   │   └── void_protocol/     # Anchor program with Arcium
│   │       └── src/lib.rs     # queue_computation, callbacks
│   ├── Anchor.toml
│   └── Arcium.toml
├── packages/
│   └── void-sdk/              # TypeScript SDK (npm package)
│       └── src/index.ts
└── app/                       # Demo UI (Next.js)
```

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.85+
- [Solana CLI](https://docs.solana.com/cli/install) 2.3.0+
- [Anchor](https://www.anchor-lang.com/) 0.32.1
- [Arcium CLI](https://docs.arcium.com/) 0.5.4+
- Docker (for Arcium localnet)

### Build

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/void-sdk.git
cd void-sdk

# Build MPC circuits and Anchor program
cd anchor
arcium build
anchor build

# Install SDK dependencies
cd ../packages/void-sdk
npm install
npm run build
```

### Deploy to Devnet

```bash
cd anchor
solana config set --url devnet
anchor deploy
```

## 💡 Usage

```typescript
import { VoidClient } from '@yurei/void-sdk';

const client = new VoidClient({
  programId: '9oqbvYkKhFA2EFrJKGujRqzHnCRGuGnzTD6dyXuxo6oo',
  cluster: 'devnet'
});

// Initialize private account (encrypted on-chain)
await client.initPrivateAccount();

// Coming soon:
// await client.deposit(100_000_000); // 100 USDC → 100 cUSDC
// await client.swapConfidential(cUSDC, cSOL, amount);
// await client.transferPrivate(recipient, amount);
```

## 🔧 Technical Details

### Encrypted Instructions (MPC Circuits)

```rust
// anchor/encrypted-ixs/src/lib.rs
pub struct PrivateAccount {
    pub owner_lo: u128,   // Pubkey split for MPC
    pub owner_hi: u128,
    pub balance: u64,
    pub token_mint: u64,
}

#[instruction]
pub fn init_account(mxe: Mxe) -> Enc<Mxe, PrivateAccount> {
    // Returns encrypted state owned by MXE
}
```

### Anchor Program with Arcium

```rust
// anchor/programs/void_protocol/src/lib.rs
#[arcium_program]
pub mod void_protocol {
    pub fn create_private_account(...) -> Result<()> {
        // Queue MPC computation
        queue_computation(ctx.accounts, ...)?;
    }

    #[arcium_callback(encrypted_ix = "init_account")]
    pub fn init_account_callback(...) -> Result<()> {
        // Store encrypted result on-chain
    }
}
```

## 🎯 Hackathon Tracks

| Track | Prize | How We Qualify |
|-------|-------|----------------|
| **Arcium Prize** | $10,000 | End-to-end private DeFi with MPC |
| **Privacy Tooling** | $15,000 | Developer SDK abstracting Arcium complexity |

## 📋 Requirements Checklist

- [x] Open source code
- [x] Solana integration with Arcium
- [x] Deploy to Devnet
- [ ] Demo video (3 min max)
- [x] Documentation

## 🛣️ Roadmap

- [x] **Phase 1**: Core encrypted account structure
- [ ] **Phase 2**: Deposit/Withdraw with C-SPL tokens
- [ ] **Phase 3**: Confidential swaps
- [ ] **Phase 4**: SDK npm package release

## 👥 Team

**Project Yurei** — Building the Dark DeFi infrastructure

## 📄 License

MIT License — See [LICENSE](LICENSE) for details.

---

<p align="center">
  <b>🌑 Privacy is not a feature. It's a right.</b>
</p>
