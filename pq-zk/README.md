# pq-zk — a NIST ACVP-conformant ML-DSA-65 verifier (reference / learning)

> A from-scratch, **NIST-ACVP-conformant** ML-DSA-65 (FIPS-204) signature verifier in Rust,
> validated 45/45 against the official NIST vectors. This is a **reference / learning
> implementation** — not audited, not production, and **not a novel solution**.
>
> ⚠️ Research only. Do not guard real funds with this.

## Context: on-chain post-quantum verification is already an active area

This started as an exploration of verifying post-quantum (ML-DSA-65 / FIPS-204) signatures on
Ethereum. It turns out that is a well-developed space with strong existing work:

- **[ZKNox ETHDILITHIUM](https://github.com/ZKNoxHQ/ETHDILITHIUM)** — direct on-chain ML-DSA /
  Dilithium verification in Solidity that **fits within the per-transaction gas limit** (a
  NIST-compliant version and an EVM-tuned variant). See also
  [ETHFALCON](https://github.com/ZKNoxHQ/ETHFALCON).
- **zkVM approaches** — Dilithium signature verification has been implemented as a zkVM guest
  (e.g. in SP1), with cheap on-chain proof checking.
- **[EIP-8051](https://eips.ethereum.org/EIPS/eip-8051)** — a proposed native ML-DSA precompile.

So this folder is **not** a new or cheaper solution to that problem. What it is: a clean,
from-scratch reference verifier with reproducible NIST-ACVP conformance — useful for learning, and
potentially for independently conformance-testing other implementations.

## What's here

```
pq-zk/
├── verifier/    ACVP-conformant ML-DSA-65 verify logic (Rust), validated 45/45 vs NIST ACVP
└── vectors/     conformance notes + an independent-oracle KAT (mldsa65_kat.json)
```

## Status

- ✅ **NIST ACVP-conformant (45/45, three interface modes)** — see
  [`vectors/ACVP_CONFORMANCE.md`](vectors/ACVP_CONFORMANCE.md). Validated against the official NIST
  ACVP ML-DSA-sigVer-FIPS204 vectors and against an independent-oracle signature (RustCrypto
  `ml-dsa`).

## License

MIT (see the repository [`LICENSE`](../LICENSE)).
