# WS Lending Protocol

> A decentralized lending protocol on Solana built with Anchor. Supports SOL and USDC deposits and borrows using share-based accounting. 🚧 Work in Progress

A decentralized lending protocol built on Solana using the Anchor framework. This protocol enables users to deposit SOL and USDC as collateral and borrow other assets, implementing a share-based accounting system similar to Compound/Aave.

> ⚠️ **Work in Progress**: This project is currently under active development. Features may be incomplete and the codebase is subject to change.

## Features

- **Share-Based Accounting**: Uses a share-based system where deposits and borrows are converted to shares that track interest accrual
- **Dual Asset Support**: Currently supports SOL and USDC with separate banks for each asset
- **Oracle Integration**: Integrates with Pyth Network for real-time price feeds to calculate collateral values
- **Collateral-Based Borrowing**: Users can borrow assets based on their deposited collateral with configurable LTV ratios
- **Liquidation Mechanisms**: Includes liquidation thresholds, bonuses, and close factors for managing risk
- **Program Derived Addresses (PDAs)**: Uses PDAs for deterministic account addresses and program-controlled accounts
- **Anchor Framework**: Built with Anchor 0.30.1 for type-safe Solana program development

## Architecture

### Core Components

#### Bank
The `Bank` account stores the state for each supported asset (SOL and USDC):
- `authority`: The program authority
- `mint_address`: The token mint address this bank manages
- `total_deposits`: Total amount of tokens deposited (increases with interest)
- `total_deposits_share`: Total shares issued (stays constant, enables interest accrual)
- `total_borrowed`: Total amount of tokens borrowed (increases with interest)
- `total_borrowed_share`: Total borrow shares issued (stays constant, enables interest accrual)
- `liquidation_threshold`: Loan-to-value ratio at which liquidation is triggered
- `liquidation_bonus`: Percentage bonus for liquidators
- `liquidation_close_factor`: Percentage of collateral that can be liquidated
- `max_ltv`: Maximum loan-to-value ratio for borrowing
- `interest_rate`: Interest rate used for accruing interest on deposits (applied exponentially)
- `last_updated`: Timestamp of last update

#### User
The `User` account tracks individual user positions:
- `owner`: The user's wallet address
- `deposited_sol` / `deposited_sol_share`: SOL deposit amounts and shares
- `borrowed_sol` / `borrowed_sol_share`: SOL borrow amounts and shares
- `deposited_usdc` / `deposited_usdc_share`: USDC deposit amounts and shares
- `borrowed_usdc` / `borrowed_usdc_share`: USDC borrow amounts and shares
- `usdc_mint_address`: USDC mint address for the user
- `last_updated`: Timestamp of last update

### Instructions

#### `initialize_bank`
Initializes a new bank for SOL or USDC token mint. Creates:
- A PDA `Bank` account derived from the mint address
- A PDA treasury token account for holding deposits

#### `init_user`
Initializes a user account for tracking deposits and borrows. Creates:
- A PDA `User` account derived from the user's wallet address

#### `deposit`
Allows users to deposit SOL or USDC into the protocol. This instruction:
- Transfers tokens from the user's token account to the bank's treasury account
- Calculates and assigns shares based on the current exchange rate
- Updates the user's deposit balances and shares
- Updates the bank's total deposits
- Uses share-based accounting where the first deposit sets the initial exchange rate

#### `withdraw`
Allows users to withdraw SOL or USDC from the protocol. This instruction:
- Accrues interest on deposits using exponential growth based on time elapsed and interest rate
- Calculates the current value per share after interest accrual
- Validates that the user has sufficient balance (including accrued interest)
- Calculates shares to withdraw based on the current exchange rate
- Transfers tokens from the bank's treasury account back to the user's token account
- Uses PDA signing for the treasury account transfer
- Updates the user's deposit balances and shares
- Updates the bank's total deposits and shares
- Creates the user's token account if it doesn't exist (init_if_needed)

#### `borrow`
Allows users to borrow SOL or USDC against their collateral. This instruction:
- Retrieves real-time price feeds from Pyth Network oracle
- Calculates total collateral value using accrued interest on deposits and current market prices
- Validates that the borrow amount doesn't exceed the liquidation threshold
- Calculates and assigns borrow shares based on the current exchange rate
- Transfers tokens from the bank's treasury account to the user's token account
- Uses PDA signing for the treasury account transfer
- Updates the user's borrow balances and shares
- Updates the bank's total borrowed and borrowed shares
- Initializes the borrow pool if this is the first borrow (sets exchange rate to 1:1)
- Creates the user's token account if it doesn't exist (init_if_needed)

## Project Structure

```
ws_lending/
├── programs/
│   └── ws_lending/
│       └── src/
│           ├── lib.rs              # Main program entry point
│           ├── errors.rs           # Custom error definitions
│           ├── constants.rs        # Constants (Pyth feed IDs, max age)
│           ├── states/
│           │   ├── mod.rs
│           │   └── states.rs       # Account state definitions (Bank, User)
│           └── instructions/
│               ├── mod.rs
│               ├── admin.rs        # Admin instructions (initialize_bank, init_user)
│               ├── deposit.rs      # Deposit instruction
│               ├── withdraw.rs     # Withdraw instruction
│               └── borrow.rs       # Borrow instruction
├── tests/
│   └── ws_lending.ts               # Test suite
├── Anchor.toml                     # Anchor configuration
├── Cargo.toml                      # Rust workspace configuration
└── package.json                    # Node.js dependencies
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.30.1)
- [Yarn](https://yarnpkg.com/getting-started/install) (v3.1.1+)
- Node.js (v16+)

### Key Dependencies

- **Anchor Lang/SPL**: Core framework for Solana program development
- **Pyth Solana Receiver SDK**: Oracle integration for real-time price feeds

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd ws_lending
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Development

### Build the Program
```bash
anchor build
```

### Run Tests
```bash
anchor test
```

### Deploy to Localnet
```bash
# Start a local validator
solana-test-validator

# In another terminal, deploy
anchor deploy
```

### Deploy to Devnet
```bash
# Set cluster to devnet
solana config set --url devnet

# Deploy
anchor deploy
```

## Configuration

The program is configured via `Anchor.toml`:

- **Program ID**: `FDpT1vmBWwJvEf7RbDAy1STwUs4AUEXraB6wEnj5bVRd`
- **Cluster**: Configured for `localnet` (change to `devnet` or `mainnet` as needed)
- **Wallet**: Defaults to `~/.config/solana/id.json`

## How It Works

### Share-Based Accounting

The protocol uses a share-based accounting system:

1. **Exchange Rate**: `total_deposits / total_deposits_share`
2. **On Deposit**: User receives shares proportional to their deposit
3. **Interest Accrual**: `total_deposits` increases while `total_deposits_share` stays constant
4. **On Withdrawal**: Shares are converted back to tokens using the current exchange rate

This allows interest to accrue to depositors without updating individual user accounts continuously.

### Example

1. Initial state: `total_deposits = 1000`, `total_deposits_share = 1000` (exchange rate = 1.0)
2. User deposits 100 tokens: receives 100 shares. New state: `total_deposits = 1100`, `total_deposits_share = 1100`
3. Interest accrues: `total_deposits = 1200`, shares stay at 1100 (exchange rate = 1.09)
4. User withdraws: 100 shares convert to ~109 tokens (100 * 1200 / 1100)

## Security Considerations

⚠️ **This is a development/educational project. Do not use in production without:**

- Comprehensive security audits
- Extensive testing
- Proper access controls
- Emergency pause mechanisms
- Rate limiting and oracle price feeds for liquidations

## License

ISC

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built as part of a Solana bootcamp, learning Anchor framework and DeFi protocol development on Solana.

