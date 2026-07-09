// Generate the Etherscan "Solidity (Standard-JSON-Input)" verification file for the deployed
// BharatCoinToken. It recursively bundles the token source + every OpenZeppelin file it
// imports, with the EXACT compiler settings used at deploy time (optimizer on / 200 runs, no
// other overrides), so Etherscan reproduces the identical bytecode. Output: verify-token-input.json
const fs = require("fs");
const path = require("path");

const CONTRACTS = path.join(__dirname, "..", "contracts");
const NM = path.join(__dirname, "..", "node_modules");
const ENTRY = "BharatCoinToken.sol";

const sources = {};
const IMPORT_RE = /import\b[^;]*?["']([^"']+)["']/g;

function diskPath(key) {
  return key.startsWith("@") ? path.join(NM, key) : path.join(CONTRACTS, key);
}
function resolve(key) {
  if (sources[key]) return;
  const content = fs.readFileSync(diskPath(key), "utf8");
  sources[key] = { content };
  const dir = path.posix.dirname(key);
  let m;
  IMPORT_RE.lastIndex = 0;
  while ((m = IMPORT_RE.exec(content)) !== null) {
    const p = m[1];
    const childKey = p.startsWith(".") ? path.posix.normalize(dir + "/" + p) : p;
    resolve(childKey);
  }
}

resolve(ENTRY);

const input = {
  language: "Solidity",
  sources,
  settings: {
    optimizer: { enabled: true, runs: 200 },
    outputSelection: { "*": { "*": ["abi", "evm.bytecode.object"] } },
  },
};

const out = path.join(__dirname, "..", "verify-token-input.json");
fs.writeFileSync(out, JSON.stringify(input, null, 2));
console.log("wrote", out);
console.log("sources bundled:", Object.keys(sources).length);
Object.keys(sources).sort().forEach((k) => console.log("  -", k));
