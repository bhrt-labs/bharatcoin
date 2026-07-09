# pq-zk — zkVM-provable post-quantum (ML-DSA-65) signature verification

> **Standalone R&D / public good.** An open, NIST-ACVP-conformant ML-DSA-65 (FIPS-204)
> verifier that a zkVM can prove, so a post-quantum signature can be checked **cheaply
> on-chain** (~300k gas) despite the ~16.78M per-transaction gas cap (EIP-7825). Deliberately
> independent of any single coin — every chain will need this pre-quantum.
>
> ⚠️ Research, not production. Not audited. Do not guard real funds with this.

## Why this exists

Direct on-chain ML-DSA verification is ~87M gas — it does not fit in one transaction on
modern L2s. A zkVM lets us run a *proven* verifier off-chain and check a succinct proof
on-chain for cheap. See the full plan in [`../docs/ZK_ROADMAP.md`](../docs/ZK_ROADMAP.md).

## What's here / planned

```
pq-zk/
├── README.md              # this file
├── verifier/              # ACVP-conformant ML-DSA-65 verify logic (Rust), validated 45/45
│                          #   vs the official NIST ACVP vectors
├── vectors/               # conformance notes + an independent oracle KAT (mldsa65_kat.json)
├── guest/                 # (planned) zkVM guest program wrapping the verifier
├── host/                  # (planned) host/prover driver (local or hosted prover, e.g. Bonsai)
└── onchain/               # (planned) Solidity proof-verifier + integration example
```

## Status

- ✅ The verifier logic exists and is **NIST ACVP-conformant (45/45, three interface modes)** —
  included here in [`verifier/`](verifier/): `acvp_fips204.rs` (the ACVP runner) and
  `verify_fips204.rs` (an independent-oracle check).
- ⬜ M1 educational zk prototype (hosted prover)
- ⬜ M2 verifier as a zkVM guest
- ⬜ M3 on-chain proof verification (~300k gas target)
- ⬜ M4 public-good packaging + grant + specialist audit

## Provenance of the verifier

The verify logic is validated against the official NIST ACVP ML-DSA-sigVer-FIPS204 vectors —
it accepts every valid signature and rejects every should-reject case across internal /
external-pure / external-mu modes. That conformance is the foundation a zk proof would attest
to. See [`vectors/ACVP_CONFORMANCE.md`](vectors/ACVP_CONFORMANCE.md).

## License

MIT (see the repository [`LICENSE`](../LICENSE)) — this is meant to be reused.
