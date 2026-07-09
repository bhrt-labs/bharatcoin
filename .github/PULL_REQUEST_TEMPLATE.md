## What & why

<!-- What does this change, and why? Link any related issue (e.g. "Closes #12"). -->

## Checklist

- [ ] This is **not** a security vulnerability (those are reported privately — see SECURITY.md).
- [ ] I ran the local checks and they pass:
  - [ ] `node scripts/deploy-token.js` (compile + smoke test)
  - [ ] `node scripts/gen-verify-input.js` then `git diff --exit-code verify-token-input.json` (reproducibility)
- [ ] I did **not** change the pinned `@openzeppelin/contracts` / `solc` versions (or I explain why below).
- [ ] No secrets, private keys, or `.env` files are included.
- [ ] I agree my contribution is licensed under the repository's MIT License.

## How I tested

<!-- Commands run, output, or reasoning. -->
