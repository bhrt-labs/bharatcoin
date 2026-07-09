// Clean, token-only deploy for the shipped BharatCoin product: the fixed-supply BHRT token
// + the founder vesting wallet. The community / staking / treasury allocations (83%) go to
// your Safe multisig; the founder 17% goes into a 1-yr-cliff + 3-yr-linear vesting wallet.
// NO PQ wallet / verifier here — that stayed as R&D (see pq-zk/). Safe secures the funds.
//
//   Dry run (local, no network, no keys):   node scripts/deploy-token.js
//   Real testnet (Sepolia):
//     RPC_URL="https://rpc.sepolia.org" PRIVATE_KEY="0xYOUR_TESTNET_KEY" \
//     SAFE="0xYourSafeAddress" FOUNDER="0xVestingBeneficiary" \
//     node scripts/deploy-token.js
//
// SAFETY: run on a TESTNET with a THROWAWAY key. Never paste a real-funds key.
const solc = require("solc");
const fs = require("fs");
const path = require("path");
const ganache = require("ganache");
const { ethers } = require("ethers");

const CONTRACTS = path.join(__dirname, "..", "contracts");
const NM = path.join(__dirname, "..", "node_modules");
const YEAR = 365 * 24 * 60 * 60;

function findImport(p) {
  try {
    const full = p.startsWith("@") ? path.join(NM, p) : path.join(CONTRACTS, p.replace(/^\.\//, ""));
    return { contents: fs.readFileSync(full, "utf8") };
  } catch (e) { return { error: "File not found: " + p }; }
}
function compile() {
  const files = ["FounderVesting.sol", "BharatCoinToken.sol"];
  const sources = {};
  for (const f of files) sources[f] = { content: fs.readFileSync(path.join(CONTRACTS, f), "utf8") };
  const out = JSON.parse(solc.compile(JSON.stringify({
    language: "Solidity", sources,
    settings: { optimizer: { enabled: true, runs: 200 }, outputSelection: { "*": { "*": ["abi", "evm.bytecode.object"] } } },
  }), { import: findImport }));
  const hard = (out.errors || []).filter((e) => e.severity === "error");
  if (hard.length) { hard.forEach((e) => console.log(e.formattedMessage)); throw new Error("compile failed"); }
  const pick = (file, name) => ({ abi: out.contracts[file][name].abi, bytecode: "0x" + out.contracts[file][name].evm.bytecode.object });
  return { vesting: pick("FounderVesting.sol", "FounderVesting"), token: pick("BharatCoinToken.sol", "BharatCoinToken") };
}

async function getContext() {
  if (process.env.RPC_URL && process.env.PRIVATE_KEY) {
    const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
    const deployer = new ethers.Wallet(process.env.PRIVATE_KEY, provider);
    return { provider, deployer, dry: false };
  }
  const gp = ganache.provider({ logging: { quiet: true }, wallet: { totalAccounts: 3 } });
  const provider = new ethers.BrowserProvider(gp);
  const deployer = await provider.getSigner(0);
  return { provider, deployer, dry: true };
}

async function main() {
  const art = compile();
  const { provider, deployer, dry } = await getContext();
  const me = await deployer.getAddress();
  console.log(dry ? "== DRY RUN (local ganache) ==" : "== LIVE TESTNET DEPLOY ==");
  console.log("deployer:", me);

  // The Safe multisig receives community/staking/treasury (83%). Defaults to deployer in dry run.
  const safe = process.env.SAFE || me;
  // Founder-vesting beneficiary (who can pull the 17% as it unlocks). Defaults to deployer.
  const founder = process.env.FOUNDER || me;

  const now = (await provider.getBlock("latest")).timestamp;
  const start = BigInt(now + YEAR);   // 1-year cliff
  const duration = BigInt(3 * YEAR);  // + 3-year linear release

  const deploy = async (a, ...args) => {
    const f = new ethers.ContractFactory(a.abi, a.bytecode, deployer);
    return (await f.deploy(...args)).waitForDeployment();
  };

  const vesting = await deploy(art.vesting, founder, start, duration);
  const vestingAddr = await vesting.getAddress();

  // community, staking, treasury all -> the Safe multisig.
  const token = await deploy(art.token, vestingAddr, safe, safe, safe);
  const tokenAddr = await token.getAddress();

  console.log("\nDeployed:");
  console.log("  FounderVesting :", vestingAddr, `(beneficiary ${founder}; 1yr cliff + 3yr linear)`);
  console.log("  BharatCoinToken:", tokenAddr);
  console.log("  Safe (treasury):", safe, "(holds community+staking+treasury = 83%)");

  const total = await token.totalSupply();
  const vBal = await token.balanceOf(vestingAddr);
  const sBal = await token.balanceOf(safe);
  console.log("\nsmoke checks:");
  console.log("  totalSupply    :", total.toString(), total === 100000000000000000n ? "OK (1B, 8dp)" : "MISMATCH");
  console.log("  founder vested :", vBal.toString(), vBal === (total * 1700n) / 10000n ? "OK (17%)" : "MISMATCH");
  console.log("  Safe balance   :", sBal.toString(), sBal === (total * 8300n) / 10000n ? "OK (83%)" : "MISMATCH");
  if (dry) console.log("\ndry run complete.");
  else console.log("\nLIVE deploy complete. Verify the token + vesting source on the block explorer next.");
  process.exit(0);
}
main().catch((e) => { console.error(e); process.exit(1); });
