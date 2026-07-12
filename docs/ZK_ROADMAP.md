# On-chain post-quantum signature verification — context & prior art

> **Honest status:** this file originally proposed making on-chain ML-DSA verification cheap via a
> zkVM. On closer investigation, on-chain post-quantum verification is already an active,
> well-developed area, and the approaches below already exist. This document now records that
> context, so the [`pq-zk/`](../pq-zk/README.md) verifier is understood as a **reference / learning
> implementation**, not a novel solution.

## The landscape (existing work)

- **Direct on-chain verification already fits the gas budget.**
  [ZKNox ETHDILITHIUM](https://github.com/ZKNoxHQ/ETHDILITHIUM) implements ML-DSA / Dilithium
  verification in Solidity that runs within the EIP-7825 per-transaction gas limit (a
  NIST-compliant version and an EVM-tuned keccak256 variant). See also
  [ETHFALCON](https://github.com/ZKNoxHQ/ETHFALCON).
- **zkVM approaches exist.** Dilithium signature verification has been implemented as a zkVM guest
  (e.g. in SP1), producing a succinct proof that is checked cheaply on-chain.
- **A native precompile is proposed.** [EIP-8051](https://eips.ethereum.org/EIPS/eip-8051)
  (ML-DSA verification precompile) targets a native, low-gas path.
- **Account-abstraction integration** is being explored on ethresear.ch (e.g. asanso's series) so
  smart accounts can adopt post-quantum signatures without a protocol change.

## What `pq-zk/` is (and isn't)

- It **is**: a clean, from-scratch, NIST-ACVP-conformant (45/45) ML-DSA-65 verifier in Rust — a
  reference implementation with reproducible conformance results.
- It is **not**: a novel or cheaper on-chain verifier. The problem it originally set out to solve
  is already addressed by the work above.

## A possible honest direction

Lattice verifiers are subtle (NTT, rejection sampling, encoding edge cases), so an independent
ACVP-conformance / differential test across the existing implementations could be genuinely useful
— **if** it isn't already being done. That is the only direction here worth pursuing further.
