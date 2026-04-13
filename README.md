# Stellar Royalty Splitter

A Soroban smart contract on the Stellar network that automatically distributes NFT sale proceeds among multiple collaborators based on predefined percentage allocations.

---

## How it works

1. Deploy the contract
2. Call `initialize` with collaborator addresses and their shares (in basis points)
3. When a sale occurs, funds are sent to the contract address
4. Call `distribute` — funds split instantly, on-chain, with no intermediaries

Shares are expressed in **basis points** (1 bp = 0.01%). They must sum to **10,000** (100%).

---

## Project structure

```
├── src/lib.rs              # Contract source
├── tests/integration_test.rs
├── scripts/deploy.sh       # Build + deploy helper
└── Cargo.toml
```

---

## Prerequisites

| Tool          | Install                                    |
| ------------- | ------------------------------------------ |
| Rust          | https://rustup.rs                          |
| wasm32 target | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI   | `cargo install --locked stellar-cli`       |

---

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## Test

```bash
cargo test
```

---

## Deploy to Testnet

```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

---

## Contract API

### `initialize(collaborators: Vec<Address>, shares: Vec<u32>)`

Sets up the revenue split. Can only be called once.

- `collaborators` — list of recipient wallet addresses
- `shares` — basis-point allocation per collaborator (must sum to 10,000)

### `distribute(token: Address, amount: i128)`

Transfers `amount` of `token` from the contract to all collaborators proportionally.

### `get_collaborators() → Vec<Address>`

Returns all registered collaborator addresses.

### `get_share(collaborator: Address) → u32`

Returns the basis-point share for a given address.

---

## Example: 3-way split

```bash
# 50% artist / 30% musician / 20% animator
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize \
  --collaborators '["GARTIST...","GMUSICIAN...","GANIMATOR..."]' \
  --shares '[5000,3000,2000]'
```

---

## Rounding

Integer division is used for each collaborator's payout. Any rounding dust (1–2 stroops) is assigned to the last collaborator in the list to ensure the full amount is always distributed.

---

## Roadmap

- [ ] Dynamic royalty adjustments via governance
- [ ] Secondary market resale royalty hooks
- [ ] Role-based contributor management
- [ ] Dashboard UI for earnings tracking

---

## License

MIT
