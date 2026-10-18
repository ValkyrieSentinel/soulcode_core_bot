# SoulCode Core

Ultra-efficient Web3 token distribution toolkit. Save up to 45% on gas fees using Yul/Assembly core, Rust calldata encoding, and Zig memory-level validation.

## Architecture
- **contracts/**: Safe Solidity airdrop contract (pause, gas stipend, two-step ownership).
- **src/**: Rust Axum API with rate limiting, Alloy calldata encoder, and EN/UK localization.
- **src/p2p_validator.zig**: Zero-allocation FFI signature validation.
- **frontend/**: React Telegram Web App with native theming and haptic feedback.

## Stack
Rust, Zig, Solidity, React, TypeScript.

## License
MIT. Part of the ValkyrieSentinel ecosystem.
