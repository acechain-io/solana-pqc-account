//! ZK-ACE Circuit tooling for ACE Account Kit.
//!
//! Uses dev.zk-ace's Stwo Circle STARK backend (post-quantum secure).
//!
//! ## Circuit IDs
//!
//! Since Stwo uses transparent setup (no trusted setup required), the circuit
//! identity is established by the constraint set itself. The circuit_id is:
//!   SHA-256(circuit_label) — used to identify which AIR produced a proof.
//!
//! ## Usage
//!
//!   cargo run --release --bin extract_circuit_id
//!   cargo run --release --bin prove_test

pub mod circuit_id;

pub use circuit_id::{ZK_ACE_CIRCUIT_ID, ZK_OWNERSHIP_CIRCUIT_ID, compute_circuit_id};
