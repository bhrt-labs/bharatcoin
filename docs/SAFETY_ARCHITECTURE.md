# BharatCoin Wallet — Safety Architecture (Defense in Depth)

> **Honest status:** this design uses **Safe** (formerly Gnosis Safe) and its audited modules
> — so the wallet code itself is *already professionally audited and secures billions*. You
> inherit that. What still stands between this and holding real user funds: a public
> **testnet** period, a **bug bounty**, and **legal review** (token classification / local
> rules). This document is the design, not a certificate of safety.

## Why Safe, not a custom wallet

A wallet holds money, so **every unaudited line of custom wallet code is a catastrophic
risk**, and the dangerous part is usually the *glue* between features — that's where real
wallets have been drained (the Parity multisig freeze; several social-recovery hacks). A
solo builder with no audit budget cannot make custom wallet code safe, because the audit that
would make it safe is unaffordable.

**Safe already provides every layer we want, audited, holding tens of billions of dollars.**
So we don't write a wallet — we *configure* Safe. This is the single highest-leverage safety
decision in the whole project.

## The layers (no single point of failure)

| # | Layer | What it stops | How (Safe / audited module) |
|---|-------|---------------|------------------------------|
| 1 | **Multisig 2-of-3** | One stolen/lost key ≠ loss | Safe core: 3 owner keys, threshold 2. |
| 2 | **Daily spending limit** | A breach draining everything at once | Safe **Allowance Module** — per-token cap per day; above it needs full multisig. |
| 3 | **Time-delay on big moves** | A theft you don't catch instantly | **Zodiac Delay Module** — large txs queue and execute only after a cooldown (e.g. 24–48h). |
| 4 | **Cancel / freeze during the delay** | Stopping an in-flight theft | Owners veto a queued tx during its cooldown; guardians can raise the threshold / pause. |
| 5 | **Social recovery** | A permanently lost key | Guardians rotate owners after a **recovery delay** (so guardians can't act instantly or collude silently). |

Recommended starting parameters (tune to taste):
- Owners: **3** (e.g. your main key, a hardware key, a backup) · Threshold: **2**.
- Daily allowance: a modest cap for routine spend; anything larger takes 2-of-3 + the delay.
- Delay cooldown: **24 hours** to start.
- Guardians: **2–3 trusted parties** · Recovery delay: **several days**.

## Threat → mitigation

| If this happens… | You're protected because… |
|------------------|----------------------------|
| One key is phished/stolen | It's only 1 of 2 needed — the thief still can't move funds. |
| Attacker steals a key AND initiates a big transfer | It's over the daily limit → needs 2-of-3, and it sits in the delay where you cancel it. |
| A co-signer key is compromised | Still need a 2nd honest key; guardians can rotate the bad key out. |
| You lose a key | Guardians recover after the recovery delay — funds not lost. |
| Something looks wrong right now | Freeze: veto queued txs / raise threshold during the cooldown window. |

## The post-quantum "recovery key" (experimental, NOT shipped)

The post-quantum key you built can, in the future, serve as an **extra recovery factor**
(a guardian-style key using ML-DSA). It is **NOT** shipped as custom on-chain code here,
because any custom module holding funds would need its own audit we can't fund yet. It stays
a documented, testnet-only experiment until there's a budget or a native PQ precompile.
See [ZK_ROADMAP.md](ZK_ROADMAP.md).

## Honest limits (read this)

- Safe's *code* is audited; **your configuration and your keys are your responsibility.**
  Bad key hygiene (all keys on one laptop, all guardians = you) defeats the design.
- This is not "unhackable." Nothing is. It's **layered so no single failure loses everything.**
- Before real user funds: **public testnet + bug bounty + legal review.** No shortcuts.

## Setting it up

Safe is already deployed on all major L2 testnets — you don't deploy wallet contracts, you
create a Safe and enable modules. See [SAFE_SETUP.md](SAFE_SETUP.md) for the step-by-step
(Safe web app for the multisig, then the Allowance + Delay modules).
