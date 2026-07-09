# Post-quantum signature verification does not fit in an EVM transaction — measurements, and a zkVM path

*bhrt-labs · contact: bhrt.labs@proton.me*

## TL;DR

- Verifying **one ML-DSA-65** (FIPS-204, the ratified Dilithium3) signature fully on-chain measured **~87M gas** in my Solidity experiment — down from an initial ~326M after optimization, including an assembly Keccak-f1600 host contract.
- **EIP-7825** (live since Fusaka, Dec 2025) caps a single transaction at **~16.78M gas (2^24)**. Direct on-chain PQ verification is therefore **~5x over the per-tx cap** — it cannot run, on L1 or on modern L2s that inherit the cap.
- Optimization plateaus: the algorithm is dominated by SHAKE/ExpandA and NTT work that is structurally expensive in the EVM. This is not a "write better Solidity" problem.
- What exists today: a from-scratch **Rust ML-DSA-65 verifier, validated 45/45 against the official NIST ACVP ML-DSA-sigVer-FIPS204 vectors** across all three signature-interface modes, including negative (should-reject) cases, plus an independent-oracle KAT. Open source (MIT): https://github.com/bhrt-labs/bharatcoin (folder `pq-zk/`).
- Proposed path: run that same tested verifier as a **zkVM guest** (RISC Zero or SP1), prove `verify(pk, msg, sig) = true` off-chain, verify the succinct proof on-chain for roughly **~300k gas** (target) — which fits the cap.
- This is unaudited R&D by a pseudonymous solo builder. Feedback, re-runs of the vectors, and collaborators are what I'm here for.

## 1. The wall: ML-DSA-65 on-chain vs the per-transaction gas cap

I implemented full ML-DSA-65 verification in Solidity to measure what "just verify it on-chain" actually costs. First working version: **~326M gas**. After optimization passes — including moving the Keccak-f1600 permutation into a dedicated assembly host contract — it came down to **~87M gas** for a single signature verification.

EIP-7825 caps any single transaction at 2^24 = **16,777,216 gas**, and it has been live since Fusaka (December 2025). So the optimized figure is still roughly **5x over the hard cap**. There is no block to "fill more of" — the transaction simply cannot execute. This holds on L1 and on the modern L2s that adopt the cap.

Why optimization plateaus:

- **SHAKE dominates.** ML-DSA verification requires SHAKE-128/256 as an XOF: expanding the seed ρ into the public matrix **A** (ExpandA — for ML-DSA-65 that is a 6×5 matrix of degree-255 polynomials, filled by rejection sampling from SHAKE-128 output), plus the message representative and the challenge hash. That means hundreds of Keccak-f1600 permutations per verification. The EVM's `keccak256` opcode is a *finalized hash*, not an incremental permutation, so it cannot serve a XOF's squeeze phase — you must run Keccak-f1600 in EVM code, and even in tight assembly, manipulating the 25×64-bit lane state through 256-bit words makes each permutation expensive — and a verification needs hundreds of them (ExpandA alone fills 30 polynomials by rejection sampling).
- **NTT and polynomial arithmetic gas-cost poorly.** The verifier performs number-theoretic transforms and pointwise multiplications over ℤ_q (q = 8380417) on vectors of 256-coefficient polynomials. Every 23-bit modular operation occupies a full 256-bit EVM word and pays full word-op prices; there is no vectorization, and memory expansion charges grow with the working set.
- **The inputs are big.** An ML-DSA-65 signature is ~3.3 KB and the public key ~2 KB, so calldata and hashing costs are non-trivial before the math even starts — but they are a rounding error next to ExpandA + NTT.

I do not claim my 87M figure is the global optimum — I'd genuinely like to see someone do better — but the dominant costs are structural. Nobody should expect a further 5x from Solidity cleverness alone.

Every EVM chain hits this wall the moment the ecosystem migrates post-quantum. That makes cheap on-chain PQ verification a shared-infrastructure problem, which is why this work is unbundled as a standalone public good rather than a feature of any one project.

## 2. What exists today: an ACVP-conformant Rust verifier

The repo contains a from-scratch **Rust ML-DSA-65 verifier** validated against the official NIST ACVP known-answer vectors (`usnistgov/ACVP-Server`, `ML-DSA-sigVer-FIPS204/internalProjection.json`):

- **45/45 correct accept/reject decisions**, across all three signature-interface modes NIST defines: **internal** 15/15, **external-pure** 15/15, and **internal-externalMu** 15/15.
- Every `testPassed: false` (should-reject) vector is correctly rejected — the negative cases are the ones that matter for a verifier.
- **preHash mode is intentionally skipped** (this verifier targets pure-mode signing; no OID/preHash path).
- An **independent-oracle KAT**: the verifier accepts a signature produced by the RustCrypto `ml-dsa` crate — code it shares nothing with — and rejects tampered variants. This demonstrates conformance to the *standard*, not merely round-trip consistency with my own signer.

Code and conformance notes: https://github.com/bhrt-labs/bharatcoin — see `pq-zk/verifier/` (`acvp_fips204.rs`, `verify_fips204.rs`) and `pq-zk/vectors/ACVP_CONFORMANCE.md`. The full ACVP JSON (~4.5 MB) is not vendored; download it from NIST and pass its path to the runner. **I'd welcome anyone re-running the vectors independently** — that is the entire point of publishing this.

## 3. Why a zkVM, not a hand-written lattice circuit

If direct execution doesn't fit, the natural move is succinct verification: prove off-chain that `verify(pk, msg, sig) = true` and check only the proof on-chain. Two ways to get there:

| | zkVM (RISC Zero / SP1) | Hand-written circuit (Circom / Halo2 / Noir) |
|---|---|---|
| Reuses the ACVP-tested Rust verifier | Yes — compile it as the guest | No — full rewrite from scratch |
| Frontier-research risk | Lower — the novel machinery is a third-party layer built and reviewed by a dedicated team, though the zkVM itself joins the trusted base with its own (evolving) assurance story | High — NTT, rejection sampling, and SHAKE in a custom circuit are easy to get catastrophically wrong |
| Proof generation cost | Heavy (RAM-hungry; prover time to be measured and published) | Lighter, if done expertly |
| On-chain verification cost | Cheap (~300k gas target) | Cheap |
| Realistic for a solo builder | Plausible (especially via a hosted prover) | No |

A hand-optimized lattice circuit would almost certainly prove faster. But it means re-implementing exactly the arithmetic that the ACVP suite exists to catch mistakes in, inside a formalism where bugs are silent. Reusing code that already passes 45/45 conformance — and letting the zkVM attest to its execution — trades prover efficiency for a much smaller hand-written crypto surface (while adding the zkVM itself to the trusted base). For a solo builder, I think that trade is not close. I'm open to being argued out of it.

## 4. Proposed milestones

- **M1 — Educational prototype (hosted prover ok).** Prove a tiny computation end-to-end (e.g. "I know x such that hash(x) = y") and verify it on-chain. *Acceptance:* a real proof, generated and verified, mechanics understood. Not the ML-DSA circuit.
- **M2 — Verifier as a zkVM guest.** Compile the Rust `verify_fips204` verifier as a guest program; prove **one real ACVP vector** off-chain. *Acceptance:* proof of a genuine NIST vector verifying, with cycle counts and prover resource numbers published.
- **M3 — On-chain proof verification.** Deploy the zkVM's Solidity proof verifier; verify the M2 proof on-chain. *Acceptance:* measured gas **under the ~16.78M per-tx cap**, target ~300k.
- **M4 — Public-good packaging.** Standalone repo, a spec/EIP-style write-up of the interface, and pursuit of a **specialist audit** of the guest + on-chain verifier. *Acceptance:* someone other than me can reproduce every step from the docs.

## 5. Open questions for this community

1. **Which zkVM?** RISC Zero vs SP1 for a SHAKE/NTT-heavy guest — does either's Keccak precompile/acceleration materially change proving cost for ML-DSA specifically? Real-world data welcome.
2. **Proof aggregation.** One proof per signature won't scale for, say, a PQ-guarded rollup bridge. What's the current best practice for batching N `sigVer` claims into one on-chain verification?
3. **Does a future native precompile obsolete this?** If an ML-DSA precompile ships on L1, yes — and that would be a **good outcome**. This work is the bridge (and the tested reference/spec groundwork) for the years before that happens, and the measurement above is, I hope, useful evidence *for* such an EIP.
4. **Account abstraction.** What's the cleanest integration for a PQ-guarded account — ERC-4337 validation calling a proof verifier, a 7702-style path, or something else? Is a proof-of-signature acceptable inside validation gas limits?
5. **Is 87M reproducible/beatable?** If someone has materially better direct-EVM numbers for full ML-DSA-65 verification, I'd like to compare notes.

## 6. Limitations, and a call for collaborators

To be explicit about status: this is **R&D, unaudited, and not production**. The measured artifact today is the conformant Rust verifier plus the Solidity gas experiment; the zkVM milestones are proposed, not done. The Solidity experiment code behind the ~326M → ~87M numbers is **not yet in the public repo** — I'll publish it, with methodology, alongside the M4 write-up; until then, treat those figures as my report rather than an independently reproduced result. Proving is RAM- and time-heavy — it's the *verification* of the proof that's cheap — so real deployments likely need hosted or delegated proving, with its own trust considerations. Before anything here guards funds, the zkVM guest and the on-chain verifier need their own specialist audit. And I'm a pseudonymous solo builder, which is exactly why the code, the vectors, and the numbers are public: don't trust me — re-run them.

If you work on zkVMs, lattice crypto, or AA and want to poke holes in any of this — or collaborate on M2/M3 — I'd genuinely value the review: **bhrt.labs@proton.me**, or issues/PRs on the repo. MIT-licensed; reuse is the point.
