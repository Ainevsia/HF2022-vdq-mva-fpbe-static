// SPDX-License-Identifier: MIT

pragma solidity =0.8.12;

contract Challenge {
    uint constant fee = 100000;
    uint constant max_code_size = 0x80;

    event SendFlag(address);

    function solve() public {
        uint answer;
        bool success;
        bytes memory result;

        assembly {
            answer := extcodesize(caller())
        }
        require(answer < max_code_size);

        (success, result) = msg.sender.staticcall{gas:fee}("");
        answer = uint(bytes32(result));
        require(success && answer == 1);

        (success, result) = msg.sender.staticcall{gas:fee}("");
        answer = uint(bytes32(result));
        require(success && answer == 2);

        emit SendFlag(msg.sender);
    }
}
