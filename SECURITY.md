# Security Policy & Bug Bounty — BharatCoin

Thanks for helping keep BharatCoin safe. This is a **recognition-based** program (no cash
required to participate) for a **pre-launch, testnet-only** project. Please read the rules —
especially the **eligibility** section — before reporting.

## How to report (private, responsible disclosure)

- **Do NOT open a public issue for a vulnerability.** Report privately to:
  **bhrt.labs@proton.me** (PGP key optional). We aim to acknowledge within a few days.
- Give us reasonable time to fix before any public disclosure.
- Never test against anyone else's funds or infrastructure. **Testnet only.**

## What we care about (in scope)

- The **BharatCoin token** contract (fixed-supply ERC-20) and any first-party scripts.
- Our **Safe module configuration** and any first-party glue we publish.
- Anything that could let someone **mint, steal, freeze, or lock** tokens, or **break the
  supply/vesting rules**.

## Out of scope

- **Safe / Gnosis Safe core contracts and Zodiac modules** — these are audited upstream;
  report those to their own programs, not here.
- Third-party infrastructure (RPCs, wallets, block explorers, faucets).
- **Gas / optimization suggestions, style, or "best-practice" nitpicks** with no exploit.
- **Theoretical issues with no demonstrated security impact.**
- Testnet-only griefing that has no mainnet analog; social engineering; phishing.

## ⚠️ Eligibility — READ THIS (anti-spam, anti-AI-slop, anti-false-alarm)

To keep this program useful for a solo maintainer, the following are **automatically closed,
unreviewed, and ineligible**:

1. **No working proof, no report.** Every submission MUST include a concrete, **reproducible
   proof-of-concept** — exact steps, a transaction hash on the testnet, or runnable code that
   demonstrates real impact. "I think this might be vulnerable" is not a report.
2. **No raw AI-agent / LLM output.** Reports that are unmodified or lightly-edited output of an
   AI agent / LLM / "autonomous security agent," submitted without a **human who verified it
   and can discuss it**, are **out of scope and closed without response.** A real person must
   stand behind the finding and have actually reproduced it.
3. **No automated-scanner dumps.** Output from vulnerability scanners/linters **without a
   demonstrated, working exploit** is not eligible.
4. **No false alarms / hallucinated bugs.** Claims that reference functions, files, or
   behaviors that **do not exist** in the code, or that misread how the code works, are
   ineligible. Check the actual source first.
5. **Impact required.** You must show *what an attacker gains.* No impact = not eligible.
6. **One clear issue per report**, in English, with the affected file/line and the fix if you
   have one. Duplicates and already-documented limitations are ineligible.

Repeated low-quality or AI-generated submissions may be **blocked from the program.**

## Rewards (recognition-based)

Because this is pre-launch with **no real funds at stake**, rewards are recognition, not cash:

- **Security Hall of Fame** credit (in this repo) for every valid, in-scope report.
- Public thanks + a written acknowledgment you can cite.
- *(Optional, at maintainer discretion)* a small symbolic reward for exceptional findings.

**When real money is ever involved (mainnet):** this policy will be replaced by a **funded**
bounty with cash tiers *and* an independent professional audit — because only then does the
reward need to beat the incentive to exploit. Until then: honest testing, honest credit.

## Hall of Fame

*(Valid reporters will be listed here. Be the first.)*
