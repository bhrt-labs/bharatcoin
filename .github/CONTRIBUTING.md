# Contributing to BharatCoin

Thanks for your interest! BharatCoin is a **testnet-only, pre-launch, open-source** project — a
fixed-supply ERC-20 token, a Safe-based wallet design, and post-quantum verifier R&D. There is
**no real money involved**, and nothing here is an offer or investment.

Contributions of all kinds are welcome: bug fixes, tests, docs, and the post-quantum / zk R&D.

## Ground rules

- Be respectful — see our [Code of Conduct](CODE_OF_CONDUCT.md).
- **Security bugs do NOT go here.** Never open a public issue or PR for a vulnerability — report
  it privately (see [SECURITY.md](../SECURITY.md), or use **"Report a vulnerability"** in the
  repo's **Security** tab).
- One focused change per pull request. Explain the *why*, not just the *what*.

## Getting set up

```bash
git clone https://github.com/bhrt-labs/bharatcoin
cd bharatcoin
npm install
```

## The checks CI runs (run them locally before a PR)

```bash
# Compile the contracts + run local smoke checks (supply, 17% vesting, 83% to the Safe):
node scripts/deploy-token.js

# Confirm the committed Etherscan source bundle still reproduces byte-for-byte:
node scripts/gen-verify-input.js
git diff --exit-code verify-token-input.json
```

Both must pass. The reproducibility check matters: `verify-token-input.json` must keep matching
the on-chain verified source, so **do not change the pinned `@openzeppelin/contracts` (5.0.0) or
`solc` (0.8.20) versions** unless you also intend to redeploy.

## The deployed contracts are immutable

`contracts/BharatCoinToken.sol` and `contracts/FounderVesting.sol` are already deployed and
source-verified on Sepolia. Please treat them as **frozen** — a PR that changes their compiled
bytecode can't apply to the live token. Improvements to scripts, docs, tests, and the `pq-zk/`
R&D are the most useful.

## Quality bar (please respect a solo maintainer's time)

- Include a clear description and, for a fix, how you verified it.
- No unmodified AI-agent / LLM output submitted without a human who understands and can discuss
  it — a real person should stand behind every change.
- No automated-scanner dumps without a demonstrated, reproducible issue.

## License

By contributing, you agree your contributions are licensed under the repository's
[MIT License](../LICENSE).
