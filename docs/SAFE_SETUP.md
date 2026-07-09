# Setting Up Your Defense-in-Depth Wallet with Safe (Step by Step)

> This configures the layered wallet from [SAFETY_ARCHITECTURE.md](SAFETY_ARCHITECTURE.md)
> using **Safe** — already-audited infrastructure deployed on every major L2 testnet. You do
> **not** deploy or write wallet code. Do this on a **testnet first** (no real funds).

## What you'll need

- A browser wallet (e.g. MetaMask) with **3 separate keys/accounts** for the owners
  (ideally on different devices — a laptop key, a hardware wallet, a backup).
- A little **testnet ETH** on your chosen L2 (Base Sepolia or Arbitrum Sepolia) from a faucet.
- ~30 minutes.

## Part A — The multisig (no code)

1. Go to the **Safe app** (app.safe.global) and connect your main wallet.
2. Choose your testnet (e.g. **Base Sepolia**).
3. **Create a new Safe** → add your **3 owner addresses** → set **threshold = 2**.
   *(Now any transfer needs 2 of the 3 keys. One stolen key can't move funds.)*
4. Send a little test token/ETH into the Safe and try a transfer — notice it needs a **2nd
   confirmation** before it executes. That's layer 1 working.

## Part B — Daily spending limit (Allowance module)

The Allowance module lets a single key spend up to a **daily cap** without the full 2-of-3,
while anything above the cap still needs the multisig.

- In the Safe app: **Settings → Modules → Spending limits** (Allowance module).
- Add a beneficiary (e.g. your main key) and a **per-day cap** per token.
- *Effect:* routine small spending is convenient; a breach can't drain everything at once —
  large moves need the full multisig (Part A) and the delay (Part C).

## Part C — Time-delay on big moves (Zodiac Delay module)

The Delay module makes queued transactions wait a cooldown before they can execute — giving
you a window to **cancel** a theft in progress.

- Enable the **Zodiac Delay Modifier** on your Safe (via the Zodiac app / SDK — see below).
- Set a **cooldown** (start with **24 hours**) and an expiration.
- *Effect:* a large transfer is *queued*, not instant. If it's not something you did, any
  owner cancels it during the cooldown. This is your "catch it in time" layer + freeze.

## Part D — Social recovery (guardians)

So a lost key never means lost funds:

- Add **2–3 guardians** (trusted people/devices) via a Safe recovery setup (the Safe app's
  Recovery feature, or a Zodiac recovery module).
- Set a **recovery delay** (several days) so guardians can't act instantly or collude
  silently — you'd see a pending recovery and could veto it.

## Optional — automate with the Safe SDK (protocol-kit)

If you prefer scripts over clicking, the Safe SDK does all of the above programmatically.
Install and use `@safe-global/protocol-kit` (v6+) and `@safe-global/api-kit`:

```js
// npm i @safe-global/protocol-kit @safe-global/api-kit ethers
import Safe from "@safe-global/protocol-kit";
// 1) Deploy/predict a Safe with 3 owners, threshold 2:
const safe = await Safe.init({
  provider: process.env.RPC_URL,
  signer: process.env.PRIVATE_KEY,
  predictedSafe: { safeAccountConfig: { owners: [OWNER1, OWNER2, OWNER3], threshold: 2 } },
});
const tx = await safe.createSafeDeploymentTransaction();
// ...send tx with your signer to deploy the Safe on the testnet...
// 2) Enable modules (Allowance, Zodiac Delay) via safe.createEnableModuleTx(moduleAddress)
//    using the module addresses from the Safe/Zodiac deployments for your chain.
```

Module contract packages (already audited, fetchable from npm):
- Safe core: `@safe-global/safe-smart-account` · Allowance module ships with Safe deployments.
- Zodiac (Delay/recovery modifiers): `@gnosis-guild/zodiac`.

## Before real money (do NOT skip)

1. Run everything on **testnet** for a while; try to break your own setup.
2. **Open-source** your token + any scripts; run a **bug bounty**.
3. Get a **US legal opinion** on the token (classification/Howey; FinCEN & state
   money-transmitter rules; IRS tax treatment).
4. Only then consider mainnet — and even then, start small.

*The wallet code is audited (it's Safe). Your **keys and configuration** are your
responsibility: keep the 3 owner keys on different devices, and pick guardians you trust.*
