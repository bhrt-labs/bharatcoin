# BharatCoin (BHRT)

[![CI](https://github.com/bhrt-labs/bharatcoin/actions/workflows/ci.yml/badge.svg)](https://github.com/bhrt-labs/bharatcoin/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-testnet%20%C2%B7%20pre--launch-orange.svg)](#honest-disclosures)
[![Solidity](https://img.shields.io/badge/solidity-0.8.20-363636.svg)](contracts/BharatCoinToken.sol)
[![Sepolia: source verified](https://img.shields.io/badge/Sepolia-source%20verified-brightgreen.svg)](https://sepolia.etherscan.io/address/0x36afd10e97f16d4dec3d05691d4276c408e0aca1#code)
[![Security: report a bug](https://img.shields.io/badge/security-report%20a%20bug-red.svg)](SECURITY.md)

> **Status: pre-launch, testnet-only.** BharatCoin is **not live for real funds**. Nothing in
> this repository is an offer to sell, financial advice, or a promise of value. See
> [Honest disclosures](#honest-disclosures).

**BharatCoin (BHRT)** is a **fixed-supply ERC-20 token** for an Ethereum Layer-2, paired with a
**defense-in-depth wallet built on [Safe](https://safe.global)** — the audited multisig that
secures tens of billions of dollars. Instead of running a risky solo-operated blockchain, it
inherits Ethereum's battle-tested security. The focus is **honesty and safety over hype.**

## Why it's different

- **Supply can never be inflated.** 1,000,000,000 BHRT are minted once, at deployment. There is
  **no mint function** — not for anyone, including the deployer.
  ([contract](contracts/BharatCoinToken.sol))
- **The team cannot dump.** The founder's 17% is locked in a public vesting contract:
  **1-year cliff, then 3-year linear release** (4-year total). ([contract](contracts/FounderVesting.sol))
- **A wallet design with no single point of failure.** A documented **Safe** configuration you
  set up — 2-of-3 multisig, a daily spending limit, a time-delay on large transfers
  (cancellable), an emergency freeze, and guardian-based recovery — so no single stolen key can
  drain funds. The wallet code is already professionally audited (it's Safe). *The deployed
  testnet Safe is a 2-of-3; the additional modules are a [setup guide](docs/SAFE_SETUP.md), not
  yet enabled.* ([full design](docs/SAFETY_ARCHITECTURE.md))

## Live on Ethereum Sepolia (testnet)

Deployed and **source-verified** — anyone can confirm there is no hidden mint or backdoor:

| Contract | Address | |
|---|---|---|
| BharatCoin token (BHRT) | `0x36afd10e97f16d4dec3d05691d4276c408e0aca1` | [verified source ↗](https://sepolia.etherscan.io/address/0x36afd10e97f16d4dec3d05691d4276c408e0aca1#code) |
| FounderVesting (17%, locked) | `0x88e8A6B003D22c4742B7Df586437Be3eaD797768` | [↗](https://sepolia.etherscan.io/address/0x88e8A6B003D22c4742B7Df586437Be3eaD797768) |
| Safe 2-of-3 (holds 83%) | `0x88cf54fE89B7d8Cf576A4032A9c2afbC1c15ac13` | [↗](https://sepolia.etherscan.io/address/0x88cf54fE89B7d8Cf576A4032A9c2afbC1c15ac13) |

> These are **testnet** addresses. BHRT has **no monetary value.**

## Tokenomics

| Allocation | Share | Amount (BHRT) | Notes |
|---|---|---|---|
| Community / Ecosystem | 64% | 640,000,000 | Distribution & ecosystem grants (reserve) |
| Staking / Rewards | 16% | 160,000,000 | Reserve bucket (see note) |
| Founder | 17% | 170,000,000 | **Vested**: 1-yr cliff + 3-yr linear |
| Treasury | 3% | 30,000,000 | Operations, grants |
| **Total** | **100%** | **1,000,000,000** | Fixed · 8 decimals · no inflation |

Ticker **BHRT** · Decimals **8** · Standard **ERC-20** (+ Permit, + Burnable).

> **Reserve allocations only** — not a promise of yield, rewards, or returns. **No staking,
> rewards, or yield program is offered, planned, or built,** and holders are not promised any
> profit.

## Repository layout

```
contracts/                BharatCoinToken.sol, FounderVesting.sol  (the deployed, verified source)
scripts/                  deploy-token.js, gen-verify-input.js, verify-helper.js
verify-token-input.json   exact Standard-JSON-Input used to verify on Etherscan
docs/                     one-pager, safety architecture, Safe setup, ZK roadmap, legal questions
pq-zk/                    standalone post-quantum (ML-DSA-65) verifier R&D — a public good
SECURITY.md               responsible-disclosure / bug-bounty policy
```

## Verify or reproduce it yourself

```bash
npm install
# Re-generate the exact Standard-JSON-Input that verifies on Etherscan:
node scripts/gen-verify-input.js
# Read the live token straight from the chain (name, supply, balances):
node scripts/verify-helper.js
# Deploy your own copy to a testnet (throwaway key; see the script header for details):
RPC_URL=... PRIVATE_KEY=0x... SAFE=0x... FOUNDER=0x... node scripts/deploy-token.js
```

Compiler: **Solidity 0.8.20**, optimizer **enabled / 200 runs** (matches the verified bytecode).
Reproducing the bundle byte-for-byte requires the pinned deps (`@openzeppelin/contracts` **5.0.0**,
`solc` **0.8.20**); the committed [`verify-token-input.json`](verify-token-input.json) is the
authoritative copy that matches the on-chain source.

## Post-quantum: honest R&D, not a shipped claim

We built a **from-scratch, NIST ACVP-conformant (45/45) post-quantum ML-DSA-65 verifier** — but
we do **not** market BharatCoin as "quantum-safe," and it is **not** a shipped
feature (verifying a post-quantum signature on-chain costs far more than a single transaction's
gas budget allows). The work continues **openly as a public good** in [`pq-zk/`](pq-zk/README.md)
and [`docs/ZK_ROADMAP.md`](docs/ZK_ROADMAP.md), for the day a cheap on-chain proof (zk) makes it
real. See the [one-pager](docs/BHARATCOIN_ONEPAGER.md) for the plain-English version.

## Honest disclosures

- BharatCoin runs on Ethereum-family infrastructure secured by **classical** cryptography. It is
  **not "quantum-safe,"** and no crypto is "100% safe."
- The wallet's **code** is audited (it's Safe); **your keys and configuration are your
  responsibility.**
- This is **testnet, pre-launch, and unaudited as a whole.** Nothing here is an offer, a
  security, or investment advice.
- **Before any real funds:** a public testnet period, a bug bounty, an **independent security
  audit**, and a **US securities/legal review** (see [docs/LEGAL_QUESTIONS.md](docs/LEGAL_QUESTIONS.md)).
  No token sale is planned until those are done.

## Security

Found a bug? Please read [SECURITY.md](SECURITY.md) and report it privately. This is a
recognition-based, testnet-only program with strict quality rules.

## License

[MIT](LICENSE). Reuse is encouraged — especially the [`pq-zk/`](pq-zk/README.md) verifier, which
is meant to be a shared public good.
