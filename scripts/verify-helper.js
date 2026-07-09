// Read-only helper: confirms the deployed BHRT token and derives the ABI-encoded constructor
// arguments needed for Etherscan verification (founderVesting, community, staking, treasury).
// The vesting address is read from the chain (the 170M mint recipient). No keys, read-only.
//   TOKEN=0x... SAFE=0x... node scripts/verify-helper.js
const { ethers } = require("ethers");

const RPC = process.env.RPC_URL || "https://ethereum-sepolia-rpc.publicnode.com";
const TOKEN = process.env.TOKEN || "0x36afd10e97f16d4dec3d05691d4276c408e0aca1";
const SAFE = process.env.SAFE || "0x88cf54fE89B7d8Cf576A4032A9c2afbC1c15ac13";

const ABI = [
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function decimals() view returns (uint8)",
  "function totalSupply() view returns (uint256)",
  "function balanceOf(address) view returns (uint256)",
  "event Transfer(address indexed from, address indexed to, uint256 value)",
];

async function main() {
  const provider = new ethers.JsonRpcProvider(RPC);
  const token = new ethers.Contract(TOKEN, ABI, provider);

  const [name, symbol, dec, supply, safeBal] = await Promise.all([
    token.name(), token.symbol(), token.decimals(), token.totalSupply(), token.balanceOf(SAFE),
  ]);
  console.log("Token:", TOKEN);
  console.log("  name/symbol/decimals:", name, "/", symbol, "/", dec.toString());
  console.log("  totalSupply:", supply.toString(), "(1B?", supply === 100000000000000000n, ")");
  console.log("  Safe balance:", safeBal.toString(), "(83%?", safeBal === (supply * 8300n) / 10000n, ")");

  // Find the founder-vesting contract = the recipient of the 170,000,000 mint (from 0x0).
  const founderAmount = (supply * 1700n) / 10000n;
  const deployBlock = Number(process.env.DEPLOY_BLOCK || 11233240); // token deploy block (mints)
  const logs = await token.queryFilter(token.filters.Transfer(ethers.ZeroAddress, null), deployBlock - 1, deployBlock + 1);
  let vesting = null;
  for (const l of logs) {
    if (l.args.value === founderAmount) vesting = l.args.to;
  }
  console.log("\nMint recipients (from 0x0):");
  logs.forEach((l) => console.log("  ", l.args.to, "<-", l.args.value.toString()));
  console.log("\nfounderVesting (170M recipient):", vesting || "NOT FOUND (widen block range)");

  if (vesting) {
    const args = ethers.AbiCoder.defaultAbiCoder().encode(
      ["address", "address", "address", "address"],
      [vesting, SAFE, SAFE, SAFE]
    );
    console.log("\nConstructor args (founderVesting, community, staking, treasury) = (vesting, Safe, Safe, Safe)");
    console.log("ABI-ENCODED CONSTRUCTOR ARGS (paste into Etherscan, WITHOUT the leading 0x):");
    console.log(args.slice(2));
  }
}
main().catch((e) => { console.error(e.message || e); process.exit(1); });
