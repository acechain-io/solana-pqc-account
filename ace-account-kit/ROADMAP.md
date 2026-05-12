# Roadmap: ZK-ACE PQC Migration for Solana

## Vision

Replace Solana's Ed25519/secp256k1 signature-based authorization with a proof-system-agnostic, post-quantum-safe identity layer — without breaking existing accounts, wallets, or dApps.

The core insight: **identity commitments are just hashes**. By decoupling identity (`id_com = Poseidon(REV, salt, domain)`) from the proof system, Solana can upgrade its cryptographic foundation without a flag day migration.

---

## Architecture Levels

```
Level 1 ── Groth16 on-chain (current alt_bn128 syscalls)        ✅ Done
Level 2 ── STARK via off-chain aggregator (no protocol changes)  ✅ Done
Level 3 ── STARK verifier precompile (one new syscall)           ◻ Next
Level 4 ── Validator-native ZK-ACE (transaction pipeline)        ◻ Future
```

---

## Phase 0: Production-Harden Level 2 (Current)

**Goal**: Ship a working PQC-safe smart account system on Solana mainnet, without any protocol changes.

### 0.1 Real RISC Zero Integration
- [ ] Replace mock prover with `risc0-zkvm` host SDK
- [ ] Compile guest programs (`zk-ace-risc0-guest`, `zk-aggregator-guest`) to RISC-V ELF
- [ ] Generate real STARK proofs and verify journal correctness
- [ ] Pin image IDs in `vk.rs` to match compiled guest ELF hashes

### 0.2 Aggregator Production Hardening
- [ ] Persistent transaction queue (RocksDB or SQLite)
- [ ] Configurable batch interval and size thresholds
- [ ] Retry logic for failed on-chain submissions
- [ ] Metrics and monitoring (Prometheus endpoints)
- [ ] Multi-aggregator coordination (leader election or sharding)

### 0.3 Wallet & SDK
- [ ] TypeScript/JavaScript SDK for browser wallets
- [ ] `generate_attestation()` in JS (Ed25519 signing via WebCrypto)
- [ ] Wallet adapter integration (Phantom, Backpack, Solflare)
- [ ] CLI tool for account creation, key rotation, and recovery

### 0.4 Security
- [ ] Third-party audit of on-chain program
- [ ] Third-party audit of ZK-ACE circuit constraints (C1-C5)
- [ ] Formal verification of Poseidon round constants
- [ ] Relay authorization and slashing mechanism design
- [ ] Attestation replay analysis under adversarial relay models

### 0.5 Mainnet Deployment
- [ ] Deploy on-chain program to Solana mainnet
- [ ] Run aggregator infrastructure (geographically distributed)
- [ ] Publish SDK packages (npm, crates.io)
- [ ] Developer documentation and integration guides

**Milestone**: PQC-safe smart accounts available on Solana mainnet as a program. Any user can opt in. No protocol changes required.

---

## Phase 1: Performance Benchmarking & SIMD Proposal

**Goal**: Generate the data needed to justify a Solana protocol change.

### 1.1 Benchmarking
- [ ] Measure Level 2 end-to-end latency (attestation → aggregation → on-chain settlement)
- [ ] Compare per-tx cost: ZK-ACE attestation vs. Ed25519 signature vs. ML-DSA signature
- [ ] Measure aggregator throughput (txs/sec at various batch sizes)
- [ ] CU consumption analysis for `verify_aggregated` instruction
- [ ] STARK proof generation time on commodity hardware (single proof & batch)

### 1.2 Comparison with Alternative PQC Approaches

| Metric | Ed25519 (current) | ML-DSA-65 | SPHINCS+-128f | ZK-ACE L2 | ZK-ACE L3 (projected) |
|--------|-------------------|-----------|---------------|-----------|----------------------|
| Signature/proof size | 64 B | 3,309 B | 17,088 B | 64 B (attestation) | ~0 B (in-block) |
| On-chain verify cost | ~4K CU | TBD | TBD | ~200K CU (STARK) | ~50K CU (precompile) |
| PQC safe | No | Yes | Yes | Yes | Yes |
| Tx size impact | Minimal | Exceeds 1,232 B limit | Far exceeds limit | Minimal | Minimal |
| Proof-system upgradable | No | No | No | Yes | Yes |

### 1.3 SIMD Draft: STARK Verifier Precompile
- [ ] Draft SIMD (Solana Improvement and Development) proposal
- [ ] Define syscall interface: `sol_stark_verify(receipt_bytes, image_id) -> bool`
- [ ] Specify supported proof systems (RISC Zero STARK initially, extensible)
- [ ] CU cost model for STARK verification (proportional to proof size)
- [ ] Reference implementation in Solana validator (Rust)
- [ ] Submit SIMD to `solana-foundation/solana-improvement-documents`

**Milestone**: Published SIMD with benchmarking data showing ZK-ACE L3 is practical and superior to direct PQC signature schemes for Solana's constraints.

---

## Phase 2: Implement Level 3 (STARK Verifier Precompile)

**Goal**: Single-transaction PQC verification on Solana — no aggregator required for individual transactions.

### 2.1 Solana Validator Changes
- [ ] Implement `sol_stark_verify` syscall in `solana-program-library`
- [ ] STARK proof deserialization and verification (FRI-based)
- [ ] CU metering integration
- [ ] Syscall available on devnet for testing

### 2.2 On-Chain Program Update
- [ ] Add `Stark` variant to `verify_proof()` dispatcher that calls `sol_stark_verify`
- [ ] Remove journal-parsing verification logic (syscall handles it)
- [ ] Single-tx STARK execution path: user submits STARK proof directly
- [ ] Maintain backward compatibility with Groth16 and Attestation paths

### 2.3 Architecture After Level 3

```
┌─────────────┐
│   Wallet     │
│  (user app)  │
└──────┬───────┘
       │
       ├── Path A: Direct STARK proof (single tx, ~50K CU)
       │   └── sol_stark_verify syscall
       │
       ├── Path B: Attestation → Aggregator (batched, cheapest per-tx)
       │   └── aggregator submits batch STARK → sol_stark_verify
       │
       └── Path C: Groth16 (legacy, backward compat)
           └── alt_bn128 syscalls
```

Users choose the path based on their needs:
- **Path A**: Highest security, highest per-tx cost, lowest latency
- **Path B**: Lowest per-tx cost, slight latency (batch interval), requires relay trust
- **Path C**: Legacy compatibility, not PQC-safe

### 2.4 Migration Tooling
- [ ] One-click wallet migration: Ed25519 keypair → ZK-ACE identity
- [ ] Batch migration service for existing accounts
- [ ] Compatibility layer: dApps that check `is_signer` continue to work via CPI
- [ ] Explorer integration: display ZK-ACE account status and proof type

**Milestone**: Any Solana user can submit a PQC-safe transaction in a single tx. Aggregator path available for cost optimization. Legacy Groth16 still works.

---

## Phase 3: Ecosystem Adoption & Level 4 Design

**Goal**: Make ZK-ACE the default authorization model for Solana.

### 3.1 Ecosystem Integration
- [ ] Anchor framework integration: `#[account(zk_ace)]` attribute
- [ ] Token program extension: SPL Token accounts backed by ZK-ACE identity
- [ ] DeFi protocol integration guides (Marinade, Jupiter, Raydium)
- [ ] Cross-chain ZK-Ownership proofs (Ethereum, Cosmos via IBC)

### 3.2 Level 4 Design (Validator-Native)

Level 4 moves ZK-ACE verification into the transaction processing pipeline itself, replacing Ed25519 signature verification:

```
Current:     TX → Ed25519 verify → SVM execute
Level 4:     TX → ZK-ACE verify (STARK or attestation) → SVM execute
```

This requires:
- [ ] New transaction format with `proof` field replacing `signature`
- [ ] Validator-level attestation verification (relay consensus integration)
- [ ] Block-level STARK aggregation in leader schedule
- [ ] Gossip protocol changes for attestation propagation
- [ ] Backward-compatible dual-mode: Ed25519 and ZK-ACE transactions coexist

### 3.3 Post-Quantum Transition Timeline

```
Year 0   ─── Level 2 on mainnet (opt-in, no protocol changes)
Year 0.5 ─── SIMD proposal submitted with benchmarks
Year 1   ─── Level 3 on devnet (STARK precompile)
Year 1.5 ─── Level 3 on mainnet
Year 2   ─── Level 4 design finalized
Year 2.5 ─── Level 4 on devnet
Year 3   ─── Level 4 on mainnet, Ed25519 deprecated
```

This timeline is conservative. The critical insight is that **Level 2 buys time** — PQC-safe accounts exist on mainnet from day one, reducing urgency pressure on protocol-level changes.

---

## Why This Approach

### vs. Direct PQC Signature Replacement

Simply replacing Ed25519 with ML-DSA or SPHINCS+ faces fundamental problems on Solana:

1. **Transaction size**: Solana's 1,232-byte tx limit cannot fit ML-DSA signatures (3.3 KB) or SPHINCS+ signatures (17 KB). This requires a protocol-breaking tx format change.

2. **No upgrade path**: Once you pick ML-DSA, you're locked in. If ML-DSA is broken (as happened with SIKE in 2022), another painful migration is needed.

3. **No batching benefit**: Every transaction carries its own large PQC signature. No amortization possible.

ZK-ACE avoids all three: attestations are 64 bytes, the proof system is swappable, and STARK proofs amortize across batches.

### vs. Hash-Based Signatures (XMSS/LMS)

Hash-based OTS schemes are PQC-safe but stateful — signers must track which keys have been used. This is dangerous for wallets (key reuse = catastrophic) and incompatible with Solana's concurrent transaction model.

ZK-ACE's identity commitments are stateless. The nonce is part of the commitment, not the signing state.

### vs. Hybrid Schemes (Ed25519 + PQC)

Hybrid approaches (sign with both Ed25519 and a PQC scheme) double the signature size and verification cost without solving the upgrade path problem. ZK-ACE's multi-path architecture (Groth16/STARK/Attestation) provides defense-in-depth without the overhead.

---

## Key Dependencies

| Dependency | Status | Risk |
|-----------|--------|------|
| RISC Zero STARK prover | Production-ready (v1.x) | Low |
| Solana SIMD process | Established | Medium (political) |
| Poseidon hash security | Well-studied, used in Ethereum L2s | Low |
| BN254 scalar field | Standard, same as Ethereum | Low |
| Ed25519-dalek | Mature, audited | Low |
| Quantum threat timeline | NIST estimates 2030-2035 | Uncertain |

---

## Contributing

This project was built during the Solana Colosseum Hackathon. We welcome contributions in all areas:

- **Cryptography**: Circuit optimization, formal verification, alternative hash functions
- **Systems**: Aggregator performance, validator integration, networking
- **Ecosystem**: Wallet integration, SDK development, documentation
- **Research**: Security analysis, PQC threat modeling, economic incentive design

See the codebase structure:
```
programs/ace-account-kit/    # On-chain Solana program
circuits/                    # ZK circuits (Poseidon, RISC Zero guest/host, aggregator)
aggregator/                  # Off-chain batch aggregation service
sdk/ace-client-sdk/          # Rust client SDK
```
