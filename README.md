# Solana CLMM Engine

This is a personal project where I am building a Concentrated Liquidity Market Maker (Uniswap V3 style) from scratch on Solana.

The goal is to deeply understand tick math, swap logic, and how to optimize for compute units using Anchor and Rust.

### Core Architecture & Math 
Building a CLMM on Solana requires strict compute-unit optimization and precise state management. This project specifically tackles:
* **Q64.64 Fixed-Point Precision:** Unlike EVM's Q64.96, this protocol uses Q64.64 format to fit natively into Rust's `u128`, avoiding unnecessary compute overhead while maintaining accurate tick-to-price conversions.
* **Overflow Protection:** Implementing custom `U256` math structs specifically for intermediate multiplication steps to prevent `u128` overflow.
* **Bitwise Optimizations:** Utilizing bit-shifts and Most Significant Bit (MSB) logic to calculate `log2` for tick pricing without expensive iterations.

### Protocol Structure (Active Development)
```text
programs/clmm/src/
├── instructions/
│   ├── initialize_pool.rs   # Setup PDA, token mints, and fee tiers
│   └── ... (swap, open_position pending)
├── state/
│   ├── whirlpool.rs         # Global pool state
│   └── ... (tick_array, position pending)
└── math/
    ├── tick_math.rs         # Q64.64 boundaries and bitwise sqrt_price logic
    └── u256_math.rs         # Safe intermediate calculations

*Note: Active coding is temporarily paused as I am appearing for my university mid-term exams. I will resume the core math implementation and test-driven development on March 3rd.*