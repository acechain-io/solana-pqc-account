#!/usr/bin/env bash
# Deploy ACE Account Kit to Solana devnet
set -e

echo "=== Deploying to Solana Devnet ==="
cd "$(dirname "$0")/../ace-account-kit"

# Ensure wallet has SOL
echo "[1/4] Checking balance..."
solana config set --url devnet
solana airdrop 2 || echo "(airdrop may be rate-limited, ensure you have SOL)"
solana balance

# Build
echo "[2/4] Building program..."
anchor build

# Deploy
echo "[3/4] Deploying..."
anchor deploy --provider.cluster devnet

# Print program ID
PROGRAM_ID=$(anchor keys list | grep ace-account-kit | awk '{print $2}')
echo ""
echo "=== Deployed ==="
echo "Program ID: $PROGRAM_ID"
echo "Explorer:   https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
echo ""

# Run smoke tests against devnet
echo "[4/4] Running smoke tests..."
anchor test --provider.cluster devnet || echo "(some tests require localnet — use anchor test for full suite)"

echo ""
echo "=== Deployment complete ==="
