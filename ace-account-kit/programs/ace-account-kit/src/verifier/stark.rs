//! Circle STARK (Stwo) proof verification for ZK-ACE.
//!
//! ## Architecture
//!
//! Stwo Circle STARK proofs are post-quantum secure (hash-based, no elliptic curves).
//! However, full FRI verification is too expensive for a single Solana transaction today.
//!
//! This module implements Level 2 of the ZK-ACE roadmap:
//!   - Individual transactions use `execute_attested` (Ed25519 relay attestation)
//!   - The relay/aggregator runs full Stwo verification off-chain
//!   - Batch STARK receipts are settled via `verify_aggregated`
//!   - `execute` with `ProofData::Stark` accepts Stwo proof bytes and validates structure
//!
//! Level 3 (future): Add a native STARK verifier precompile syscall to Solana.
//! See: programs/ace-account-kit/ROADMAP.md and the ZK-ACE SIMD proposal.
//!
//! ## Proof Wire Format
//!
//! Stwo proof bytes are produced by `StwoEngine::prove()` (bincode-serialized).
//! A minimal receipt envelope wraps them:
//!
//!   [ circuit_id: [u8; 32] | num_inputs: u8 | inputs: N × [u8; 32] | proof: Vec<u8> ]
//!
//! The `circuit_id` identifies the ZK-ACE constraint set (hash of circuit parameters).
//! On-chain we validate: circuit_id matches, inputs match declared public inputs.

use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::verifier::types::VkType;
use crate::vk;

/// ZK-ACE Stwo receipt envelope.
///
/// Wraps Stwo proof bytes with circuit identification and committed public inputs.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ZkAceStarkReceipt {
    /// Circuit parameter hash (32 bytes).
    /// Identifies which ZK-ACE constraint set produced the proof.
    pub circuit_id: [u8; 32],
    /// Public inputs committed inside the proof.
    /// For ZK-ACE: [id_com, tx_hash, domain_fr, target, rp_com] (5 × 32 bytes)
    /// For ZK-Ownership: [id_com, solana_address, foreign_address, chain_id] (4 × 32 bytes)
    pub committed_inputs: Vec<[u8; 32]>,
    /// Raw Stwo proof bytes (bincode-serialized).
    pub proof_bytes: Vec<u8>,
}

impl ZkAceStarkReceipt {
    /// Parse from receipt bytes.
    ///
    /// Format: [ circuit_id(32) | num_inputs(1) | inputs(N×32) | proof_len(4) | proof ]
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        require!(data.len() >= 33, SolaaError::InvalidProofFormat);

        let mut circuit_id = [0u8; 32];
        circuit_id.copy_from_slice(&data[0..32]);

        let num_inputs = data[32] as usize;
        let inputs_end = 33 + num_inputs * 32;
        require!(data.len() >= inputs_end + 4, SolaaError::InvalidProofFormat);

        let mut committed_inputs = Vec::with_capacity(num_inputs);
        for i in 0..num_inputs {
            let start = 33 + i * 32;
            let mut elem = [0u8; 32];
            elem.copy_from_slice(&data[start..start + 32]);
            committed_inputs.push(elem);
        }

        let proof_len = u32::from_le_bytes([
            data[inputs_end],
            data[inputs_end + 1],
            data[inputs_end + 2],
            data[inputs_end + 3],
        ]) as usize;
        require!(
            data.len() >= inputs_end + 4 + proof_len,
            SolaaError::InvalidProofFormat
        );
        let proof_bytes = data[inputs_end + 4..inputs_end + 4 + proof_len].to_vec();

        Ok(Self { circuit_id, committed_inputs, proof_bytes })
    }
}

/// Serialize a ZkAceStarkReceipt to bytes for on-chain submission.
///
/// Called by the client SDK before submitting a STARK-authorized transaction.
pub fn serialize_receipt(receipt: &ZkAceStarkReceipt) -> Vec<u8> {
    let num_inputs = receipt.committed_inputs.len();
    let proof_len = receipt.proof_bytes.len();
    let total = 32 + 1 + num_inputs * 32 + 4 + proof_len;
    let mut out = Vec::with_capacity(total);

    out.extend_from_slice(&receipt.circuit_id);
    out.push(num_inputs as u8);
    for input in &receipt.committed_inputs {
        out.extend_from_slice(input);
    }
    out.extend_from_slice(&(proof_len as u32).to_le_bytes());
    out.extend_from_slice(&receipt.proof_bytes);
    out
}

/// Verify a STARK proof envelope on-chain.
///
/// Validation steps:
/// 1. Parse the receipt envelope
/// 2. Verify circuit_id matches the expected ZK-ACE constraint set
/// 3. Verify committed public inputs match the declared inputs
/// 4. Verify proof_bytes are non-empty (structural)
///
/// Full Stwo FRI verification is performed off-chain by the relay/aggregator.
/// Level 3 (future SIMD): native `sol_stwo_verify` precompile will enable
/// on-chain FRI verification within a single transaction.
pub fn verify_stark_proof(
    receipt_bytes: &[u8],
    public_inputs: &[[u8; 32]],
    vk_type: &VkType,
) -> Result<bool> {
    let receipt = ZkAceStarkReceipt::from_bytes(receipt_bytes)?;

    // 1. Verify circuit identity
    let expected_circuit_id = match vk_type {
        VkType::ZkAce => vk::get_zk_ace_circuit_id(),
        VkType::ZkOwnership => vk::get_zk_ownership_circuit_id(),
    };
    require!(
        receipt.circuit_id == expected_circuit_id,
        SolaaError::StarkVerificationFailed
    );

    // 2. Check committed inputs match declared inputs
    require!(
        receipt.committed_inputs.len() == public_inputs.len(),
        SolaaError::InvalidPublicInputs
    );
    for (committed, declared) in receipt.committed_inputs.iter().zip(public_inputs.iter()) {
        require!(committed == declared, SolaaError::InvalidPublicInputs);
    }

    // 3. Proof bytes must be non-empty
    require!(!receipt.proof_bytes.is_empty(), SolaaError::StarkVerificationFailed);

    // 4. On-chain: structural check only (full FRI verification via Level 3 precompile, future)
    // Off-chain (tests): accepts non-empty proof as valid; real Stwo verification is
    // done by the relay/aggregator before issuing an attestation.
    //
    // TODO (Level 3): sol_stwo_verify(circuit_id, committed_inputs, proof_bytes)

    Ok(true)
}

/// Build a ZkAceStarkReceipt from Stwo proof bytes and public inputs.
///
/// Called by the SDK after `StwoEngine::prove()`. The resulting receipt
/// is serialized via `serialize_receipt()` and passed as ProofData::Stark.
pub fn build_receipt(
    proof_bytes: Vec<u8>,
    public_inputs: &[[u8; 32]],
    vk_type: &VkType,
) -> ZkAceStarkReceipt {
    let circuit_id = match vk_type {
        VkType::ZkAce => vk::get_zk_ace_circuit_id(),
        VkType::ZkOwnership => vk::get_zk_ownership_circuit_id(),
    };

    ZkAceStarkReceipt {
        circuit_id,
        committed_inputs: public_inputs.to_vec(),
        proof_bytes,
    }
}
