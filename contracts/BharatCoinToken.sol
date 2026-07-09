// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

/**
 * @title BharatCoin (BHRT)
 * @notice Fixed-supply ERC-20 for deployment on an Ethereum Layer-2 (Path 2).
 *
 * Design goals (trust-first tokenomics):
 *  - FIXED SUPPLY, minted once at genesis. There is NO mint function afterwards, so
 *    no party — including the deployer — can ever inflate the supply. This is the
 *    single strongest signal to users/exchanges that they can't be diluted.
 *  - Transparent allocation (basis points sum to 10000):
 *      founder 17% (MUST be a vesting contract — see constructor), community 64%,
 *      staking 16%, treasury 3%.
 *  - ERC20Permit: gasless approvals (EIP-2612) for good UX on an L2.
 *  - ERC20Burnable: holders may burn, giving optional deflation.
 *
 * NOTE ON "QUANTUM RESISTANCE": this token inherits the security of the underlying
 * chain (classical secp256k1/BLS). It is therefore NOT a "quantum-safe coin". Any
 * post-quantum (Dilithium) transfer-authorization is an APP-LAYER wallet feature and
 * must be described as "quantum-resistant authorization", never as base-layer quantum
 * safety, until the base chain itself migrates to PQ signatures.
 */
contract BharatCoinToken is ERC20, ERC20Burnable, ERC20Permit {
    uint8 private constant _DECIMALS = 8;
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * (10 ** 8); // 1B BHRT, 8 decimals

    // Allocation in basis points (sum must equal 10000).
    uint256 public constant FOUNDER_BPS = 1700; // 17% — vested
    uint256 public constant COMMUNITY_BPS = 6400; // 64%
    uint256 public constant STAKING_BPS = 1600; // 16%
    uint256 public constant TREASURY_BPS = 300; // 3%

    /**
     * @param founderVesting Address that receives the founder allocation. In production
     *        this MUST be a vesting contract (e.g. OpenZeppelin VestingWallet) with a
     *        multi-year cliff+linear schedule — never a plain founder EOA — so the team
     *        cannot dump. Passing an EOA is allowed only for local testing.
     * @param community  Community/ecosystem distribution address.
     * @param staking    Staking-rewards reserve address.
     * @param treasury   Treasury address.
     */
    constructor(
        address founderVesting,
        address community,
        address staking,
        address treasury
    ) ERC20("BharatCoin", "BHRT") ERC20Permit("BharatCoin") {
        require(
            founderVesting != address(0) &&
                community != address(0) &&
                staking != address(0) &&
                treasury != address(0),
            "BHRT: zero address"
        );
        // Compile-time guarantee the split is exactly 100%.
        require(
            FOUNDER_BPS + COMMUNITY_BPS + STAKING_BPS + TREASURY_BPS == 10000,
            "BHRT: bad allocation"
        );

        uint256 founder = (MAX_SUPPLY * FOUNDER_BPS) / 10000;
        uint256 comm = (MAX_SUPPLY * COMMUNITY_BPS) / 10000;
        uint256 stake = (MAX_SUPPLY * STAKING_BPS) / 10000;
        uint256 treas = (MAX_SUPPLY * TREASURY_BPS) / 10000;
        // Route any integer-division dust to the community pool so the total minted is
        // exactly MAX_SUPPLY (no wei unaccounted for).
        uint256 dust = MAX_SUPPLY - (founder + comm + stake + treas);

        _mint(founderVesting, founder);
        _mint(community, comm + dust);
        _mint(staking, stake);
        _mint(treasury, treas);

        // Post-condition: exactly MAX_SUPPLY minted, and no mint path remains.
        assert(totalSupply() == MAX_SUPPLY);
    }

    function decimals() public pure override returns (uint8) {
        return _DECIMALS;
    }
}
