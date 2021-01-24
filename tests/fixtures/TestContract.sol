// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract TestContract {
    uint pos0;

    constructor() {
        pos0 = 11;
    }

    function solution() public view returns (uint) {
        return 42;
    }
}
