// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/finance/VestingWallet.sol";

/**
 * @title FounderVesting
 * @notice Thin, audited-base wrapper around OpenZeppelin's VestingWallet used to hold
 *         the BharatCoin founder allocation (17%) so it cannot be dumped.
 *
 * Recommended schedule: startTimestamp = deploy + 1 year, durationSeconds = 3 years.
 * VestingWallet releases linearly from `start` over `duration`, so a start one year in
 * the future yields a 1-year CLIFF (nothing releasable) followed by a 3-year linear
 * unlock — a 4-year total lock. The beneficiary (founder) calls release(token) to pull
 * whatever has vested so far; they can never access more than the schedule allows.
 *
 * Publishing this address + schedule is the concrete, on-chain proof to users and
 * exchanges that the team cannot rug the supply.
 */
contract FounderVesting is VestingWallet {
    constructor(address beneficiary, uint64 startTimestamp, uint64 durationSeconds)
        VestingWallet(beneficiary, startTimestamp, durationSeconds)
    {}
}
