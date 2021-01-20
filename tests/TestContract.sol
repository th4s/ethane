// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract TestContract {
    uint pos0;
    mapping(address => uint) pos1;

    constructor() {
        pos0 = 11;
        pos1[msg.sender] = 3;
    }

    function getPos0() public view returns (uint) {
        return pos0;
    }
}
