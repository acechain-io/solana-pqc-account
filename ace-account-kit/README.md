# ACE Layer

**Privacy-preserving account abstraction for Solana.**

No private keys. No identity leakage. One proof per batch.

---

## The Problem

Solana accounts are tied to Ed25519 keypairs. This creates three unsolved problems:

1. **No privacy** — every transaction is linked to a public key. Wallet clustering, MEV front-running, and strategy leakage are unavoidable.

2. **No account abstraction** — lose your seed phrase, lose everything. No social recovery, no session keys, no multi-device without key sharing.

3. **No post-quantum path** — Ed25519 will break under quantum computing. NIST PQC signatures (3-17 KB) don't fit in Solana's 1,232-byte transaction limit. There is no viable upgrade path without rebuilding the protocol.

## The Solution

ACE Layer replaces cryptographic signatures with **zero-knowledge identity proofs**.

```
Traditional:  User → signs with private key → signature on-chain → identity exposed
ACE Layer:    User → proves knowledge of identity → ZK proof batched → nothing exposed
```

Your identity is a Poseidon hash commitment: `id_com = Poseidon(REV, salt, domain)`. The secret (`REV`) never leaves your device. On-chain, only the commitment is stored — no public key, no linkable identity.

### How It Works

```
┌──────────────┐     attestation (64 bytes)     ┌─────────────────┐
│   Your App   │ ──────────────────────────────→ │   ACE Layer     │
│              │                                 │   (off-chain)   │
│  • No keys   │     "tx settled"                │                 │
│  • No signing│ ←────────────────────────────── │  • Verify ID    │
│  • Just API  │                                 │  • Batch txs    │
└──────────────┘                                 │  • STARK proof  │
                                                 └────────┬────────┘
                                                          │ 1 proof
                                                          ▼
                                                 ┌─────────────────┐
                                                 │   Solana L1     │
                                                 │                 │
                                                 │  • Verify STARK │
                                                 │  • Update state │
                                                 └─────────────────┘
```

## Features

| Feature | Status |
|---------|--------|
| ZK identity accounts (no private keys) | Done |
| Privacy (zero on-chain identity leakage) | Done |
| Social recovery via guardians | Done |
| Key rotation (PQC upgrade path) | Done |
| Batch transaction aggregation | Done |
| STARK proof settlement | Done |
| REST API for integration | Done |
| Post-quantum safe (RISC Zero STARK) | Done |

## Quick Start

### 1. Start the API server

```bash
cd api && cargo run
```

### 2. Create an account

```bash
curl -X POST http://localhost:3080/v1/accounts \
  -H "Content-Type: application/json" \
  -d '{"label": "Alice"}'
```

Response:
```json
{
  "account_id": "0a3f...b721",
  "secret": {
    "rev": "e4c1...8a2f",
    "salt": "7b02...d143"
  },
  "solana_address": "7Xk9...mQp3"
}
```

Save `rev` and `salt` securely. This is your identity — no seed phrase, no private key.

### 3. Send a private transaction

```bash
curl -X POST http://localhost:3080/v1/transactions \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0a3f...b721",
    "rev": "e4c1...8a2f",
    "salt": "7b02...d143",
    "to": "recipient_solana_address",
    "amount": 1000000000
  }'
```

Your identity is verified via ZK proof — never transmitted, never on-chain.

### 4. Run the interactive demo

```bash
# Terminal 1: start the API
cd api && cargo run

# Terminal 2: run the demo
cd demo && cargo run
```

## API Reference

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v1/accounts` | Create a ZK-ACE account |
| GET | `/v1/accounts/:id` | Get account info |
| POST | `/v1/transactions` | Submit a private transaction |
| GET | `/v1/transactions/:ref` | Get transaction status |
| POST | `/v1/settle` | Force batch settlement |
| POST | `/v1/accounts/:id/rotate` | Rotate identity key |
| GET | `/v1/status` | Service status |
| GET | `/health` | Health check |

## Architecture

```
ace-account-kit/
├── api/                    # REST API service (customer-facing)
├── aggregator/             # Batch aggregation engine
├── programs/               # Solana on-chain program (Anchor)
│   └── ace-account-kit/
│       └── src/
│           ├── verifier/   # Proof-system-agnostic verifier
│           │   ├── groth16.rs      # BN254 Groth16 (legacy)
│           │   ├── stark.rs        # RISC Zero STARK (PQC-safe)
│           │   └── attestation.rs  # AR-ACE relay attestation
│           └── instructions/
│               ├── execute.rs           # ZK-proof execution
│               ├── execute_attested.rs  # Attestation execution
│               └── verify_aggregated.rs # Batch STARK settlement
├── circuits/               # ZK circuits
│   ├── ace-poseidon/       # Poseidon hash (BN254)
│   ├── zk-ace-risc0-guest/ # RISC Zero guest (C1-C5 constraints)
│   ├── zk-ace-risc0-host/  # RISC Zero host prover
│   └── zk-aggregator-guest/# Batch aggregation circuit
├── sdk/                    # Rust client SDK
└── demo/                   # Interactive demo
```

## Why ACE Layer

### For Market Makers & Institutions

Your trading strategies are public on Solana. Every address you use is tracked, clustered, and front-run. ACE Layer makes your transactions unlinkable — different `id_com` per context, zero on-chain identity.

### For Wallets

"Write down these 24 words" is the worst UX in crypto. ACE Layer accounts have social recovery, key rotation, and session keys — without seed phrases.

### For Solana

Solana's 1,232-byte transaction limit means PQC signatures (3-17 KB) literally don't fit. ACE Layer is the only PQC migration path that doesn't require protocol changes. Individual transactions carry 64-byte attestations; STARK proofs are batched and amortized.

## Technical Foundation

Built on two peer-reviewed cryptographic constructions:

- **ZK-ACE** — Zero-knowledge authorization via identity commitments. 5 Poseidon hash constraints (~4K R1CS). Proof-system agnostic.
- **AR-ACE** — Attestation relay with proof-off-path aggregation. Individual transactions carry lightweight attestations; a single STARK proof validates the entire batch.

## License

TBD
