#!/usr/bin/env bash
# deploy.sh — Build and deploy the Stellar Royalty Splitter to Stellar Testnet
#
# Prerequisites:
#   - Rust + wasm32-unknown-unknown target  (rustup target add wasm32-unknown-unknown)
#   - Stellar CLI                           (cargo install --locked stellar-cli)
#   - A funded testnet identity             (stellar keys generate --global deployer)
#
# Usage:
#   chmod +x scripts/deploy.sh
#   ./scripts/deploy.sh

set -euo pipefail

NETWORK="testnet"
IDENTITY="deployer"
CONTRACT_NAME="stellar_royalty_splitter"

echo "▶ Building contract (release)..."
cargo build --target wasm32-unknown-unknown --release

WASM_PATH="target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.wasm"

echo "▶ Optimising wasm..."
stellar contract optimize --wasm "$WASM_PATH"

OPTIMISED_WASM="target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.optimized.wasm"

echo "▶ Deploying to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$OPTIMISED_WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo ""
echo "✅ Contract deployed!"
echo "   Contract ID : $CONTRACT_ID"
echo "   Network     : $NETWORK"
echo ""
echo "Next — initialize the contract:"
echo ""
echo "  stellar contract invoke \\"
echo "    --id $CONTRACT_ID \\"
echo "    --source $IDENTITY \\"
echo "    --network $NETWORK \\"
echo "    -- initialize \\"
echo "    --collaborators '[\"<ADDR_1>\",\"<ADDR_2>\",\"<ADDR_3>\"]' \\"
echo "    --shares '[5000,3000,2000]'"
