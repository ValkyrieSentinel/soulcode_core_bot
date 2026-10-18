// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract SoulCodeAirdrop {
    address public owner;
    address public pendingOwner;
    bool public paused;
    uint256 public totalDispersed;
    uint256 public totalFailed;
    uint256 public constant MAX_BATCH_SIZE = 500;
    uint256 public constant GAS_STIPEND = 50_000;

    event Dispersed(address indexed sender, uint256 count, uint256 sent, uint256 failed, uint256 gasUsed);
    event Withdrawn(address indexed to, uint256 amount);
    event Paused(address indexed by);
    event Unpaused(address indexed by);
    event OwnershipTransferStarted(address indexed previous, address indexed next);
    event OwnershipTransferred(address indexed previous, address indexed next);

    error NotOwner(); error NotPendingOwner(); error NotPaused(); error AlreadyPaused();
    error BatchTooLarge(); error BadLength(); error ZeroAddress(); error TransferFailed(); error Reentrancy();

    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;
    uint256 private _status = _NOT_ENTERED;

    modifier nonReentrant() { if (_status == _ENTERED) revert Reentrancy(); _status = _ENTERED; _; _status = _NOT_ENTERED; }
    modifier onlyOwner() { if (msg.sender != owner) revert NotOwner(); _; }
    modifier whenNotPaused() { if (paused) revert NotPaused(); _; }

    constructor() { owner = msg.sender; emit OwnershipTransferred(address(0), msg.sender); }

    function disperse() external payable onlyOwner whenNotPaused nonReentrant {
        uint256 gasStart = gasleft(); uint256 sent; uint256 failed; uint256 len;
        assembly {
            len := calldataload(0)
            if iszero(len) { mstore(0, 0x08c379a0); mstore(4, 0x20); mstore(36, 9); mstore(68, 0x426164206c656e6774680000000000000000000000000000000000000000); revert(0, 100) }
            if iszero(eq(calldatasize(), add(32, mul(len, 64)))) { mstore(0, 0x08c379a0); mstore(4, 0x20); mstore(36, 9); mstore(68, 0x426164206c656e6774680000000000000000000000000000000000000000); revert(0, 100) }
            let offset := 32; let stipend := 50000;
            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                let addr := calldataload(offset); let val := calldataload(add(offset, 32)); offset := add(offset, 64);
                let ok := call(stipend, addr, val, 0, 0, 0, 0);
                if ok { sent := add(sent, 1) }
                failed := add(failed, iszero(ok))
            }
        }
        if (len > MAX_BATCH_SIZE) revert BatchTooLarge();
        uint256 remaining = address(this).balance;
        if (remaining > 0) { (bool ok, ) = payable(owner).call{value: remaining}(""); if (!ok) revert TransferFailed(); }
        totalDispersed += sent; totalFailed += failed;
        emit Dispersed(msg.sender, sent + failed, sent, failed, gasStart - gasleft());
    }

    function withdraw() external onlyOwner whenNotPaused nonReentrant {
        uint256 bal = address(this).balance; if (bal == 0) return;
        (bool ok, ) = payable(owner).call{value: bal}(""); if (!ok) revert TransferFailed(); emit Withdrawn(owner, bal);
    }
    function transferOwnership(address newOwner) external onlyOwner {
        if (newOwner == address(0)) revert ZeroAddress(); pendingOwner = newOwner; emit OwnershipTransferStarted(owner, newOwner);
    }
    function acceptOwnership() external {
        if (msg.sender != pendingOwner) revert NotPendingOwner(); address old = owner; owner = pendingOwner; pendingOwner = address(0); emit OwnershipTransferred(old, owner);
    }
    function pause() external onlyOwner { if (paused) revert AlreadyPaused(); paused = true; emit Paused(msg.sender); }
    function unpause() external onlyOwner { if (!paused) revert AlreadyPaused(); paused = false; emit Unpaused(msg.sender); }
    receive() external payable {}
}