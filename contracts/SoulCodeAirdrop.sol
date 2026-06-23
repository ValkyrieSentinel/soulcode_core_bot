// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract SoulCodeAirdrop {
    address public immutable owner;

    constructor() {
        owner = msg.sender;
    }

    fallback() external payable {
        
        if (msg.sender != owner) {
            revert();
        }

        
        assembly {
            let len := calldataload(0)
            if iszero(len) { revert(0, 0) }

            let offset := 32

            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                let packed := calldataload(offset)
                let addr := and(packed, 0xffffffffffffffffffffffffffffffffffffffff)
                
                offset := add(offset, 32)
                let val := calldataload(offset)
                
                offset := add(offset, 32)

                // Переказ: якщо адреса отруєна, вона просто пропускається (захист від DOS)
                let success := call(gas(), addr, val, 0, 0, 0, 0)
            }
        }

        // 2. АВТО-ПОВЕРНЕННЯ ЗДАЧІ: Робимо через Solidity на адресу owner
        uint256 remainder = address(this).balance;
        if (remainder > 0) {
            (bool successRefund, ) = payable(owner).call{value: remainder}("");
            if (!successRefund) {
                revert();
            }
        }
    }

    function withdraw() external {
        require(msg.sender == owner, "Not owner");
        payable(owner).transfer(address(this).balance);
    }

    receive() external payable {}
}
