# Post-Quantum Signature Verification via zkVM — Roadmap (R&D / Public Good)

> **Honest status:** research & development. Nothing here is production-ready or a shipped
> guarantee. A novel zk proving pipeline needs its **own specialist audit** before it could
> ever guard real funds. This is the plan; the value is in doing it openly.

## The problem it solves

On-chain post-quantum (ML-DSA / FIPS-204) signature verification costs ~87M gas — **over
EIP-7825's ~16.78M per-transaction cap**, so it cannot run directly on modern L2s. Every
chain will hit this same wall as the ecosystem moves post-quantum. A way to verify a PQ
signature *cheaply* on-chain is a primitive **the whole ecosystem will need before Q-day.**

## The idea (why it's reachable)

We already have a **NIST ACVP-conformant ML-DSA-65 verifier in Rust** (validated 45/45
against the official vectors — see
[../pq-zk/vectors/ACVP_CONFORMANCE.md](../pq-zk/vectors/ACVP_CONFORMANCE.md)). A **zkVM**
(RISC Zero or SP1) can take *existing Rust code*, run it, and produce a succinct proof that it
executed correctly — **without hand-writing an exotic lattice circuit** (the frontier-research
part that is easy to get catastrophically wrong).

```
off-chain:  (pk, msg, sig) ──▶ [zkVM runs our Rust ML-DSA verify] ──▶ proof "it returned true"
on-chain:   verify(proof)  ~300k gas  ✅ fits the per-tx cap
```

So we **reuse proven code** and move the heavy work off-chain, leaving only a cheap proof
check on-chain. This is the pragmatic, honest path — not a hand-rolled circuit.

## Why a zkVM, not a hand-written circuit

| | zkVM (RISC Zero / SP1) — chosen | Hand-written circuit (Circom/Halo2/Noir) |
|---|---|---|
| Reuses our audited-in-spirit Rust verifier | ✅ yes | ❌ rewrite from scratch |
| Frontier-research risk (NTT, rejection sampling in-circuit) | avoided | ❌ high, solo-unsafe |
| Proof generation cost | heavy (minutes, RAM-hungry) | lighter |
| On-chain verification cost | cheap (~300k gas) | cheap |
| Solo-buildable | plausible (esp. via hosted prover) | no |

## Honest constraints

- The RISC Zero / SP1 toolchains are **large Rust installs + prebuilt prover binaries** that
  likely won't fetch in a restricted/offline environment. Use a **hosted prover** (e.g. RISC
  Zero's Bonsai) to generate proofs without a heavy local setup.
- Proving a full ML-DSA verify is **RAM- and time-heavy** to generate (the *verification* of
  the resulting proof is what's cheap).
- The zkVM **guest program + on-chain verifier still need a specialist audit** before funds.

## Milestones

1. **M1 — Educational prototype (hosted prover).** Prove a *tiny* computation end-to-end
   (e.g. "I know x such that hash(x)=y") and verify it on-chain (~300k gas). Goal: own a real,
   running "magic proof" and understand the mechanics. *(Not the ML-DSA circuit.)*
2. **M2 — Verifier as a zkVM guest.** Compile our Rust `verify_fips204` (from
   [`../pq-zk/verifier/verify_fips204.rs`](../pq-zk/verifier/verify_fips204.rs)) as a zkVM
   guest; prove one real ACVP vector off-chain.
3. **M3 — On-chain proof verification.** Deploy the zkVM's proof verifier; check an M2 proof
   on-chain; confirm it fits the per-tx gas cap.
4. **M4 — Package as a public good.** Standalone open-source repo + spec/EIP-style writeup;
   pursue a PQ/zk research grant; then a specialist audit before any funds.

## Repository

This lives as its own standalone project: [`pq-zk/`](../pq-zk/README.md) — deliberately
**unbundled from BharatCoin**, because an open ACVP-conformant, zkVM-provable ML-DSA verifier
is valuable to the whole ecosystem regardless of whether the coin succeeds.
