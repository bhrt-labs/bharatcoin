# BharatCoin (BHRT) — One-Pager

> **Status: testnet-grade / pre-launch.** BharatCoin is not live for real funds. Nothing in
> this document is an offer, financial advice, or a promise of value. See "Honest
> disclosures" below.

## What it is

**BharatCoin (BHRT)** is a **fixed-supply digital token** on an established Ethereum Layer-2,
paired with a **strongly-defended wallet** built on **Safe** (the audited multisig that
secures tens of billions of dollars). It inherits Ethereum's battle-tested security instead of
running a risky solo-operated network.

Its focus is **honesty and safety, not hype**: a supply that can never be inflated, a founder
allocation that's locked and can't be dumped, and a wallet with no single point of failure.

## Why it's different

- **Supply can never be inflated.** 1,000,000,000 BHRT are minted once. There is **no mint
  function** — not for anyone, including the founder. Verifiable on-chain.
- **The team cannot dump.** The founder's 17% is locked in a public vesting contract:
  **1-year cliff, then linear release over 3 years** (4-year total). Anyone can verify it.
- **A wallet with no single point of failure.** Built on **Safe** with defense in depth:
  **2-of-3 multisig**, a **daily spending limit**, a **time-delay on large transfers** (that
  can be cancelled), an **emergency freeze**, and **guardian-based recovery**. The wallet code
  is already professionally audited (it's Safe) — you inherit that.

## Tokenomics

| Allocation | Share | Amount (BHRT) | Notes |
|------------|-------|---------------|-------|
| Community / Ecosystem | 64% | 640,000,000 | Distribution & ecosystem grants (reserve) |
| Staking / Rewards | 16% | 160,000,000 | Reserve bucket (see note) |
| Founder | 17% | 170,000,000 | **Vested**: 1-yr cliff + 3-yr linear |
| Treasury | 3% | 30,000,000 | Operations, grants |
| **Total** | **100%** | **1,000,000,000** | Fixed. 8 decimals. No inflation. |

- **Ticker:** BHRT · **Decimals:** 8 · **Standard:** ERC-20 (+ Permit, + Burnable)
- **Deflation option:** holders may burn tokens; supply only ever decreases.
- **Reserve allocations only:** the community/staking buckets are **not** a promise of yield,
  rewards, or returns. No staking, rewards, or yield program is offered, planned, or built.

## The safety wallet, in plain words

Instead of one lock, your funds sit behind **several**: it takes **2 of your 3 keys** to move
money (a stolen key isn't enough), only a **capped amount** can leave per day, **large
transfers wait a day or two** and can be **cancelled**, there's a **freeze button**, and
**trusted guardians** can help you recover if a key is lost. See
[SAFETY_ARCHITECTURE.md](SAFETY_ARCHITECTURE.md).

## About "quantum" (honest)

`pq-zk/` contains a **from-scratch, NIST-conformant ML-DSA-65 verifier** — a reference / learning
implementation. We do **not** market BharatCoin as "quantum-safe," and it is **not** a shipped
feature. (On a classically-secured chain, a post-quantum key on one wallet wouldn't add real
protection anyway.)

On-chain post-quantum verification is already an active, well-developed area — see
[ZK_ROADMAP.md](ZK_ROADMAP.md) for context and links (ZKNox ETHDILITHIUM does direct on-chain
ML-DSA verification within the gas limit; zkVM approaches and a proposed precompile, EIP-8051,
exist too). This repo's verifier is a reference implementation, **not** a novel solution, and is
not part of the shipped product's safety promise.

## Honest disclosures (please read)

- BharatCoin runs on Ethereum-family infrastructure secured by classical cryptography. It is
  **not** "quantum-safe," and no crypto is "100% safe."
- The wallet's **code** is audited (Safe); **your keys and configuration** are your
  responsibility.
- **Before any real funds:** a public **testnet** period, a **bug bounty**, and a **US legal
  review** (token classification — SEC/Howey; FinCEN & state money-transmitter rules; IRS tax).

## Current build status (transparency)

- Fixed-supply token + founder vesting: **implemented, tested, and source-verified on the
  Ethereum Sepolia testnet.**
- Wallet: **Safe-based** defense-in-depth design + testnet setup guide (no custom
  money-holding code — you inherit Safe's audits).
- Post-quantum: a **from-scratch, ACVP-conformant (45/45) ML-DSA-65 reference verifier** exists in
  `pq-zk/` (learning/reference only — on-chain PQ verification is already solved elsewhere).
- **Not yet:** a funded bug bounty, an independent audit, and a legal review.
