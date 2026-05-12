# SolAA — Solana Account Abstraction with Post-Quantum ZK Authorization

**Website**: [solaa.hfi.network](https://solaa.hfi.network)  
**GitHub**: [github.com/acechain-io/solana-pqc-account](https://github.com/acechain-io/solana-pqc-account)  
**Academic foundation**: ZK-ACE (arXiv:2603.07974) · ACE-GF (arXiv:2511.20505) · AR-ACE (arXiv:2603.07982) · VA-DAR (arXiv:2603.02690)

---

## What is SolAA?

SolAA is the first account abstraction system for Solana that lets users **upgrade their key security, rotate keys, and prove cross-chain identity — without moving any assets**.

Solana's core model binds address to public key: `address = Ed25519 pubkey`. This makes any security upgrade require a full asset migration — unstaking SOL, closing ATAs, transferring NFTs, unwinding DeFi positions. The cost runs $50–200+ and hours of work. In practice, nobody does it.

SolAA breaks this coupling. A **SolAA Smart Account** is a PDA whose address is derived from `(program_id, identity_commitment)`, not from a public key. Authorization is proven via a ZK proof, not an Ed25519 signature. Rotating keys only updates the identity commitment in account state — the PDA address never changes, and all assets stay put.

---

## Three Core Features

### 1. ZK-Authorized Smart Account

SolAA Smart Accounts use ZK proofs for authorization instead of Ed25519 signatures. The authorization circuit (ZK-ACE, arXiv:2603.07974) proves knowledge of an identity root REV that matches the on-chain identity commitment `id_com`, without revealing REV.

**Circuit design (4,024 R1CS constraints, Poseidon/BN254):**

| Constraint | What it proves | Cost |
|---|---|---|
| C1 | `Poseidon(REV, salt, domain) = id_com` | 805 R1CS |
| C2 | Deterministic key derivation target | 1,200 R1CS |
| C3 | Authorization bound to TxHash + nonce | 1,615 R1CS |
| C4 | Replay prevention commitment | 400 R1CS |
| C5 | Domain separation (prevents cross-chain replay) | 4 R1CS |

**Performance (Apple M3 Pro, Groth16 / single-threaded):**

| Operation | Latency |
|---|---|
| Proof generation | 63 ms (median) |
| On-chain verification | 651 µs |
| Proof size | 128 bytes (vs. 2,420 bytes for ML-DSA-44) |

Fits in a single Solana transaction (~170K CU, within the 200K default limit).

**Why not verify ML-DSA directly on-chain?**  
ML-DSA verification requires millions of R1CS constraints for degree-256 polynomial NTTs. ZK-ACE needs 4,024 — three orders of magnitude fewer. Per-transaction on-chain data shrinks from 3,700–7,200 bytes (PQC sig + pubkey) to ~320–448 bytes.

**Security properties (formally proven in arXiv:2603.07974, see `ZK-ACE-paper.pdf`):**
- **Authorization soundness** — no PPT adversary can forge a proof without knowing REV (reduces to knowledge soundness + hash collision resistance)
- **Replay resistance** — monotonic nonce prevents reuse of any authorization token
- **Substitution resistance** — a proof for TxHash_A cannot authorize TxHash_B
- **Cross-domain separation** — a Solana proof cannot be replayed on another chain
- **Post-quantum security** — reduces to hash collision resistance only; no classical assumption vulnerable to Shor's algorithm is in the authorization path

---

### 2. In-Place Key Rotation

The PDA address is derived from `program_id` + the initial identity commitment. Rotating keys only updates `id_com` in account state. The address is unchanged. All assets stay.

| Rotation type | Traditional Solana | SolAA |
|---|---|---|
| Ed25519 → ML-DSA-44 (PQC upgrade) | New address + full migration | 1 tx, ~$0.001 |
| Compromised key → new key | New address + full migration | 1 tx, ~$0.001 |
| Single-sig → multi-sig | New address + full migration | 1 tx, ~$0.001 |
| Inheritance / key handoff | Move all assets | Update id_com |

---

### Social Recovery

Set a guardian at account creation. If you lose access:

1. Guardian calls `initiate_recovery(new_id_com)`
2. 7-day timelock begins (~1.5M slots)
3. Original owner can cancel anytime with a ZK proof
4. After timelock: anyone finalizes — done

Assets never move. Only the authorization key changes.

---

### Cross-Chain Identity (ZK-Ownership)

Prove on Solana that you own an Ethereum or Bitcoin address — without a bridge.

The circuit proves:
> "I know a single REV such that `HKDF(REV, 'sol:ed25519')` → my Solana address AND `HKDF(REV, 'eth:secp256k1')` → my Ethereum address."

Verified on-chain in ~280K CU (single transaction). No bridge. No locked assets. No trusted intermediary.

Use cases: cross-chain DeFi collateral, unified governance identity, airdrop eligibility without exposing private keys.

---

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│  Layer 1: SA-Migration  (client-side, zero on-chain cost) │
│  Import Phantom/Solflare mnemonic → REV32 Sealed Artifact │
│  Same address · AES-256-GCM-SIV encrypted · PQC-ready    │
└─────────────────────────┬────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────┐
│  Layer 2: SolAA Smart Account  (Anchor program, on-chain) │
│  PDA = f(program_id, seed_id_com)                         │
│  Auth: ZK-ACE STARK proof (Stwo, post-quantum)            │
│  initialize · execute · rotate_key · recovery · ownership │
└─────────────────────────┬────────────────────────────────┘
                          │
┌─────────────────────────▼────────────────────────────────┐
│  Layer 3: AR-ACE Relay + Aggregator  (off-chain service)  │
│  Ed25519 relay attestation → low-latency execution        │
│  Batch STARK aggregation → settled via verify_aggregated  │
└──────────────────────────────────────────────────────────┘
```

> **On-chain STARK verification:** Full FRI verification exceeds current single-transaction CU limits. SolAA uses AR-ACE relay attestation as the primary path, with off-chain batch aggregation. Native on-chain STARK verification is proposed via a Solana SIMD precompile (see ROADMAP.md).

---

## Technical Stack

| Component | Technology |
|---|---|
| On-chain program | Anchor 0.31.1 / Solana 2.x |
| ZK proof system | Circle STARK / Stwo (post-quantum, transparent setup) |
| ZK circuit | ZK-ACE (arXiv:2603.07974), 4,024 R1CS constraints |
| In-circuit hash | Poseidon (BN254) |
| Key encryption | Argon2id + AES-256-GCM-SIV |
| Key derivation | HKDF-SHA256 / BIP44 SLIP-0010 |
| PQC algorithm | ML-DSA-44 (NIST FIPS 204) |
| Frontend | Next.js 14 · Tailwind · Solana Wallet Adapter |

---

## Repository Structure

```
solana-colosseum-hackathon/
├── ZK-ACE-paper.pdf              # Academic foundation (arXiv:2603.07974)
├── PROPOSAL.md
├── VALUE_TO_SOLANA.md
│
└── ace-account-kit/
    ├── programs/ace-account-kit/ # On-chain Anchor program
    │   └── src/
    │       ├── lib.rs            # 9 instructions
    │       ├── state.rs          # SolaaAccount, RelayRegistry, OwnershipRecord
    │       ├── errors.rs         # SolaaError
    │       ├── vk.rs             # Circuit IDs
    │       ├── verifier/         # STARK verifier, attestation, public inputs
    │       └── instructions/     # All instruction handlers
    │
    ├── circuits/zk-ace-circuit/  # ZK-ACE circuit tooling (Stwo backend)
    ├── sdk/solaa-sdk/            # Client SDK — SA-Migration + proof generation
    ├── aggregator/               # Off-chain AR-ACE relay aggregator
    ├── api/                      # REST API server
    └── app/                      # React demo frontend
```

---

## Build & Run

```bash
# Prerequisites
rustup install stable
cargo install anchor-cli
agave-install init 2.3.0

# Build on-chain program
cd ace-account-kit
anchor build

# Build ZK circuit tooling (requires nightly — Stwo dependency)
cd circuits/zk-ace-circuit
cargo +nightly build --release

# Build client SDK
cd ../../sdk/solaa-sdk
cargo +nightly build

# Run on-chain tests
cd ../..
anchor test

# Start demo frontend
cd ../../website
npm install && npm run dev
```

Demo live at **[solaa.hfi.network](https://solaa.hfi.network)**

---

## Academic Foundation

| Paper | arXiv | Local copy | Role |
|---|---|---|---|
| ZK-ACE | [2603.07974](https://arxiv.org/abs/2603.07974) | `ZK-ACE-paper.pdf` | Authorization circuit · formal security proofs |
| ACE-GF | [2511.20505](https://arxiv.org/abs/2511.20505) | `ACE-GF-paper.pdf` | Identity derivation framework · REV32 format |
| AR-ACE | [2603.07982](https://arxiv.org/abs/2603.07982) | `AR-ACE-paper.pdf` | Proof-off-path relay protocol |
| VA-DAR | [2603.02690](https://arxiv.org/abs/2603.02690) | — | Decentralized address recovery |
| CT-DAP | [2603.07933](https://arxiv.org/abs/2603.07933) | — | Destroyable authorization paths |

---

## Why SolAA?

| | ERC-4337 style | Squads Multisig | **SolAA** |
|---|---|---|---|
| Asset migration required | Yes | Yes | **No** |
| Original address preserved | No | No | **Yes** |
| PQC ready | No | No | **Yes** |
| Key rotation | New address | Limited | **In-place** |
| Cross-chain identity | No | No | **ZK proof** |
| Academic backing | — | — | **5 papers** |

*Demo: [solaa.hfi.network](https://solaa.hfi.network)*
