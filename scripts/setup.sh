#!/usr/bin/env bash
# ACE Account Kit — Full Setup Script
# Installs all prerequisites and builds the project.
set -e

echo "=== ACE Account Kit Setup ==="

# 1. Rust / Anchor prerequisites
if ! command -v cargo &>/dev/null; then
  echo "[1/6] Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

echo "[1/6] Rust: $(rustc --version)"

if ! command -v anchor &>/dev/null; then
  echo "[2/6] Installing Anchor CLI..."
  cargo install --git https://github.com/coral-xyz/anchor anchor-cli --tag v0.31.1 --locked
fi
echo "[2/6] Anchor: $(anchor --version)"

if ! command -v solana &>/dev/null; then
  echo "[3/6] Installing Solana CLI..."
  sh -c "$(curl -sSfL https://release.anza.xyz/v2.3.0/install)"
fi
echo "[3/6] Solana: $(solana --version)"

# 2. Build Solana on-chain program
echo "[4/6] Building Solana program..."
cd "$(dirname "$0")/../ace-account-kit"
anchor build

# 3. Build circuits (verify circuit IDs + STARK tooling)
echo "[5/6] Building circuit tooling..."
cd circuits/zk-ace-circuit
cargo build --release
# Verify circuit IDs match constants in vk.rs
cargo test
cd ../..

# 4. Build SDK
echo "[5/6] Building client SDK..."
cd sdk/ace-client-sdk
cargo test --release
cd ../..

# 5. Frontend
echo "[6/6] Installing frontend dependencies..."
cd app
yarn install
echo "[6/6] Frontend ready: cd ace-account-kit/app && yarn dev"

echo ""
echo "=== Setup complete ==="
echo ""
echo "Next steps:"
echo "  anchor test              # Run integration tests"
echo "  anchor deploy            # Deploy to devnet"
echo "  cd app && yarn dev       # Start demo frontend"
echo "  cd ../website && npm run dev  # Start marketing website"
echo ""
echo "Run a STARK prove test:"
echo "  cd circuits/zk-ace-circuit"
echo "  cargo run --release --bin prove_test"
