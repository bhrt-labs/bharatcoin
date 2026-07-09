//! Official NIST ACVP conformance runner for the ML-DSA-65 (FIPS-204) verifier in this repo.
//!
//! Runs the ratified NIST ACVP ML-DSA-sigVer-FIPS204 known-answer vectors (the `internal`,
//! `internal-externalMu`, and `external/pure` signature interfaces) through the same verify
//! logic as `verify_fips204.rs`, and checks each accept/reject decision against NIST's
//! expected `testPassed`. This converts "matches one oracle" into "conformant to the
//! ratified standard".
//!
//! preHash groups are skipped (signing a pre-hashed digest with an OID prefix is not the
//! mode this verifier targets — it verifies a raw message representative in pure mode).
//!
//! Vectors: NIST usnistgov/ACVP-Server, ML-DSA-sigVer-FIPS204 internalProjection.json.
//! Reference source (crate deps: `sha3`, `serde_json`). Pass the ACVP vectors JSON path as
//! the first CLI argument.

use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Shake128, Shake256};
use std::fs;

const Q: i64 = 8380417;
const N: usize = 256;
const K: usize = 6;
const L: usize = 5;
const D: u32 = 13;
const TAU: usize = 49;
const GAMMA1: i64 = 1 << 19;
const GAMMA2: i64 = (Q - 1) / 32;
const BETA: i64 = 196;
const OMEGA: usize = 55;
const CTILDE: usize = 48;
const ZETA: i64 = 1753;
const PK_LEN: usize = 32 + K * 320; // 1952
const SIG_LEN: usize = CTILDE + L * 640 + OMEGA + K; // 3309

fn fq(a: i64) -> i64 { ((a % Q) + Q) % Q }
fn mulm(a: i64, b: i64) -> i64 { fq((a as i128 * b as i128 % Q as i128) as i64) }
fn powm(mut b: i64, mut e: i64) -> i64 {
    let mut r = 1i64; b = fq(b);
    while e > 0 { if e & 1 == 1 { r = mulm(r, b); } b = mulm(b, b); e >>= 1; }
    r
}
fn brv8(i: usize) -> usize {
    let mut x = (i as u32) & 0xff;
    x = (x >> 4) | (x << 4);
    x = ((x & 0xcc) >> 2) | ((x & 0x33) << 2);
    x = ((x & 0xaa) >> 1) | ((x & 0x55) << 1);
    (x & 0xff) as usize
}
fn zetas() -> [i64; 256] { let mut z = [0i64; 256]; for i in 0..256 { z[i] = powm(ZETA, brv8(i) as i64); } z }
fn ntt(a: &mut [i64; N], z: &[i64; 256]) {
    let mut k = 0; let mut len = 128;
    while len >= 1 {
        let mut start = 0;
        while start < N {
            k += 1; let zeta = z[k];
            for j in start..start + len {
                let t = mulm(zeta, a[j + len]);
                a[j + len] = fq(a[j] - t); a[j] = fq(a[j] + t);
            }
            start += 2 * len;
        }
        len >>= 1;
    }
}
fn invntt(a: &mut [i64; N], z: &[i64; 256]) {
    let mut k = 256; let mut len = 1;
    while len < N {
        let mut start = 0;
        while start < N {
            k -= 1; let zeta = fq(-z[k]);
            for j in start..start + len {
                let t = a[j];
                a[j] = fq(t + a[j + len]);
                a[j + len] = mulm(zeta, fq(t - a[j + len]));
            }
            start += 2 * len;
        }
        len <<= 1;
    }
    let ninv = powm(N as i64, Q - 2);
    for j in 0..N { a[j] = mulm(a[j], ninv); }
}
fn shake256(inp: &[u8], out: usize) -> Vec<u8> {
    let mut h = Shake256::default(); h.update(inp);
    let mut x = h.finalize_xof(); let mut o = vec![0u8; out]; x.read(&mut o); o
}
fn shake128(inp: &[u8], out: usize) -> Vec<u8> {
    let mut h = Shake128::default(); h.update(inp);
    let mut x = h.finalize_xof(); let mut o = vec![0u8; out]; x.read(&mut o); o
}
fn expand_a(rho: &[u8], i: u8, j: u8) -> [i64; N] {
    let nonce: u16 = ((i as u16) << 8) | (j as u16);
    let mut seed = rho.to_vec();
    seed.push((nonce & 0xff) as u8);
    seed.push((nonce >> 8) as u8);
    let buf = shake128(&seed, 1008);
    let mut c = [0i64; N];
    let (mut ctr, mut pos) = (0usize, 0usize);
    while ctr < N {
        let t = (buf[pos] as i64) | ((buf[pos + 1] as i64) << 8) | ((buf[pos + 2] as i64) << 16);
        let t = t & 0x7FFFFF; pos += 3;
        if t < Q { c[ctr] = t; ctr += 1; }
    }
    c
}
fn sample_in_ball(seed: &[u8]) -> [i64; N] {
    let buf = shake256(seed, 272);
    let mut c = [0i64; N];
    let mut signs = u64::from_le_bytes(buf[0..8].try_into().unwrap());
    let mut pos = 8;
    for i in (N - TAU)..N {
        let mut jj: usize;
        loop { jj = buf[pos] as usize; pos += 1; if jj <= i { break; } }
        c[i] = c[jj];
        c[jj] = if signs & 1 == 1 { Q - 1 } else { 1 };
        signs >>= 1;
    }
    c
}
fn bitunpack(bytes: &[u8], n: usize, bits: u32) -> Vec<i64> {
    let mut out = vec![0i64; n];
    let mut bitpos = 0usize;
    for i in 0..n {
        let mut c = 0i64;
        for b in 0..bits {
            let bit = (bytes[bitpos / 8] >> (bitpos % 8)) & 1;
            c |= (bit as i64) << b; bitpos += 1;
        }
        out[i] = c;
    }
    out
}
fn decompose(r: i64) -> (i64, i64) {
    let rp = fq(r);
    let mut r0 = rp % (2 * GAMMA2);
    if r0 > GAMMA2 { r0 -= 2 * GAMMA2; }
    if rp - r0 == Q - 1 { (0, r0 - 1) } else { ((rp - r0) / (2 * GAMMA2), r0) }
}
fn use_hint(h: i64, r: i64) -> i64 {
    let m = (Q - 1) / (2 * GAMMA2);
    let (r1, r0) = decompose(r);
    if h == 1 {
        if r0 > 0 { (r1 + 1).rem_euclid(m) } else { (r1 - 1).rem_euclid(m) }
    } else { r1 }
}
fn hint_unpack(y: &[u8]) -> Option<[[i64; N]; K]> {
    let mut h = [[0i64; N]; K];
    let mut index = 0usize;
    for i in 0..K {
        let end = y[OMEGA + i] as usize;
        if end < index || end > OMEGA { return None; }
        let first = index;
        while index < end {
            if index > first && y[index - 1] >= y[index] { return None; }
            h[i][y[index] as usize] = 1;
            index += 1;
        }
    }
    for i in index..OMEGA { if y[i] != 0 { return None; } }
    Some(h)
}

/// mu = H(tr || M') where tr = H(pk,64). `ctx = None` => internal (M' = M);
/// `ctx = Some(c)` => external pure (M' = 0x00 || len(c) || c || M).
fn compute_mu(pk: &[u8], m: &[u8], ctx: Option<&[u8]>) -> Vec<u8> {
    let tr = shake256(pk, 64);
    let mut mubuf = tr;
    if let Some(c) = ctx {
        mubuf.push(0x00);
        mubuf.push(c.len() as u8);
        mubuf.extend_from_slice(c);
    }
    mubuf.extend_from_slice(m);
    shake256(&mubuf, 64)
}

/// ML-DSA-65 Verify given the message-representative `mu` (supports ACVP externalMu, where
/// mu is supplied directly). Malformed-length inputs reject (never panic).
fn verify_with_mu(pk: &[u8], sig: &[u8], mu: &[u8]) -> bool {
    if pk.len() != PK_LEN || sig.len() != SIG_LEN || mu.len() != 64 { return false; }
    let z = zetas();
    let rho = &pk[0..32];
    let mut t1 = [[0i64; N]; K];
    for i in 0..K {
        let start = 32 + i * 320;
        let coeffs = bitunpack(&pk[start..start + 320], N, 10);
        t1[i][..N].copy_from_slice(&coeffs[..N]);
    }
    let c_tilde = &sig[0..CTILDE];
    let mut zpoly = [[0i64; N]; L];
    for i in 0..L {
        let start = CTILDE + i * 640;
        let u = bitunpack(&sig[start..start + 640], N, 20);
        for n in 0..N { zpoly[i][n] = GAMMA1 - u[n]; }
    }
    let h = match hint_unpack(&sig[CTILDE + L * 640..CTILDE + L * 640 + OMEGA + K]) {
        Some(h) => h,
        None => return false,
    };
    let bound = GAMMA1 - BETA;
    for i in 0..L { for n in 0..N { if zpoly[i][n].abs() >= bound { return false; } } }

    let c = sample_in_ball(c_tilde);

    let mut chat = c;
    ntt(&mut chat, &z);
    let mut zhat = [[0i64; N]; L];
    for j in 0..L { zhat[j] = zpoly[j]; ntt(&mut zhat[j], &z); }
    let mut w1_enc: Vec<u8> = Vec::new();
    for i in 0..K {
        let mut acc = [0i64; N];
        for j in 0..L {
            let a = expand_a(rho, i as u8, j as u8);
            for n in 0..N { acc[n] = fq(acc[n] + mulm(a[n], zhat[j][n])); }
        }
        let mut t1hat = [0i64; N];
        for n in 0..N { t1hat[n] = fq(t1[i][n] * (1i64 << D)); }
        ntt(&mut t1hat, &z);
        for n in 0..N { acc[n] = fq(acc[n] - mulm(chat[n], t1hat[n])); }
        invntt(&mut acc, &z);
        let base = w1_enc.len();
        w1_enc.resize(base + N * 4 / 8, 0);
        let mut bitpos = 0usize;
        for n in 0..N {
            let val = use_hint(h[i][n], acc[n]) as u32;
            for b in 0..4 {
                if (val >> b) & 1 == 1 { w1_enc[base + bitpos / 8] |= 1u8 << (bitpos % 8); }
                bitpos += 1;
            }
        }
    }
    let mut ct_in = mu.to_vec();
    ct_in.extend_from_slice(&w1_enc);
    let c_tilde2 = shake256(&ct_in, CTILDE);
    c_tilde2.as_slice() == c_tilde
}

fn dehex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "acvp_mldsa_sigver.json".to_string());
    let raw = fs::read_to_string(&path).expect("read ACVP json");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("parse ACVP json");
    println!("ACVP {} {} {}", v["algorithm"], v["mode"], v["revision"]);

    let mut ran = 0usize;
    let mut matched = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    let mut by_mode: std::collections::BTreeMap<String, (usize, usize)> = Default::default();

    for g in v["testGroups"].as_array().unwrap() {
        let ps = g["parameterSet"].as_str().unwrap_or("");
        if ps != "ML-DSA-65" { continue; }
        let iface = g["signatureInterface"].as_str().unwrap_or("");
        let prehash = g["preHash"].as_str().unwrap_or("");
        let external_mu = g["externalMu"].as_bool().unwrap_or(false);
        let mode = match (iface, prehash, external_mu) {
            ("internal", "none", false) => "internal",
            ("internal", "none", true) => "internal-extMu",
            ("external", "pure", false) => "external-pure",
            _ => continue, // skip preHash and any other mode (not used by this project)
        };
        for t in g["tests"].as_array().unwrap() {
            let expected = t["testPassed"].as_bool().unwrap();
            let pk = dehex(t["pk"].as_str().unwrap());
            let sig = dehex(t["signature"].as_str().unwrap());
            // Derive the message representative mu per interface mode.
            let mu: Vec<u8> = if external_mu {
                dehex(t["mu"].as_str().unwrap())
            } else if mode == "external-pure" {
                let ctx = dehex(t["context"].as_str().unwrap_or(""));
                compute_mu(&pk, &dehex(t["message"].as_str().unwrap()), Some(&ctx))
            } else {
                compute_mu(&pk, &dehex(t["message"].as_str().unwrap()), None)
            };
            let got = verify_with_mu(&pk, &sig, &mu);
            ran += 1;
            let e = by_mode.entry(mode.to_string()).or_insert((0, 0));
            e.0 += 1;
            if got == expected {
                matched += 1;
                e.1 += 1;
            } else {
                mismatches.push(format!(
                    "  MISMATCH tcId={} mode={} expected={} got={} reason={}",
                    t["tcId"], mode, expected, got, t["reason"]
                ));
            }
        }
    }

    println!("\nML-DSA-65 vectors run: {}", ran);
    for (mode, (n, ok)) in &by_mode {
        println!("  {:<14} {}/{} match NIST expected", mode, ok, n);
    }
    if mismatches.is_empty() {
        println!("\n*** ALL {} ML-DSA-65 ACVP vectors match NIST expected accept/reject. ***", matched);
        std::process::exit(0);
    } else {
        println!("\n{} MISMATCHES:", mismatches.len());
        for s in &mismatches { println!("{}", s); }
        std::process::exit(1);
    }
}
