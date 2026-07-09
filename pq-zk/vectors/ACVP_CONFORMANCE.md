# ML-DSA-65 — NIST ACVP Conformance

The post-quantum verifier's logic (`../verifier/acvp_fips204.rs`, `../verifier/verify_fips204.rs`)
is validated against the **official NIST ACVP known-answer vectors** for ML-DSA signature
verification (FIPS-204).

## Result

```
ML-DSA-65 vectors run: 45
  external-pure   15/15 match NIST expected   (mu = H(tr || 0x00 || len(ctx) || ctx || M))
  internal        15/15 match NIST expected   (mu = H(tr || M))
  internal-extMu  15/15 match NIST expected   (mu supplied directly)
ALL 45 ML-DSA-65 ACVP vectors match NIST expected accept/reject (incl. negative cases).
```

Every `testPassed:false` (should-reject) vector is correctly rejected, and every valid
vector is accepted — across all three signature-interface modes NIST defines.

## Vectors

- **ACVP source:** NIST `usnistgov/ACVP-Server` —
  `gen-val/json-files/ML-DSA-sigVer-FIPS204/internalProjection.json`. The full ACVP file
  (~4.5 MB) is **not** vendored here; download it from NIST and pass its path to
  `acvp_fips204.rs`.
- **Independent oracle KAT:** [`mldsa65_kat.json`](mldsa65_kat.json) — a single ML-DSA-65
  signature produced by the RustCrypto `ml-dsa` crate, used by `verify_fips204.rs` to prove the
  verifier accepts a signature it did **not** itself produce (and rejects tampering).
- preHash groups are intentionally skipped (this verifier targets the pure mode — signing a
  raw message representative; preHash/OID signing is not used).

## Scope / honesty

- This proves **standard conformance of the verifier logic**. It does **not** by itself make a
  cheap on-chain verifier — see [`../README.md`](../README.md) and
  [`../../docs/ZK_ROADMAP.md`](../../docs/ZK_ROADMAP.md) for the zkVM path.
- ACVP conformance is necessary, not sufficient: an **independent professional audit** and
  **gas optimization** remain before this could ever guard real funds.
