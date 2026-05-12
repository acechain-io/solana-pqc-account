use anchor_lang::prelude::*;
use crate::errors::SolaaError;

/// Proof-system-agnostic proof data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ProofData {
    /// RISC Zero STARK receipt (PQC-safe, transparent setup).
    Stark {
        /// Serialized RISC Zero receipt bytes.
        receipt_bytes: Vec<u8>,
    },
    /// AR-ACE relay attestation (lightweight, for proof-off-path propagation).
    Attestation {
        /// Ed25519 signature over (obj_hash || domain || nonce).
        signature: [u8; 64],
        /// Ed25519 public key of the relay node.
        relay_pubkey: [u8; 32],
        /// Hash of the object being attested.
        obj_hash: [u8; 32],
        /// Domain tag.
        domain: u64,
        /// Attestation nonce (replay prevention).
        nonce: u64,
    },
}

/// Selector for which verifying key / image ID to use.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum VkType {
    /// ZK-ACE authorization circuit (5 public inputs).
    ZkAce,
    /// ZK-Ownership cross-chain circuit (4 public inputs).
    ZkOwnership,
}

/// Unified proof verification dispatcher.
pub fn verify_proof(
    proof: &ProofData,
    public_inputs: &[[u8; 32]],
    vk_type: VkType,
) -> Result<bool> {
    match proof {
        ProofData::Stark { receipt_bytes } => {
            super::stark::verify_stark_proof(receipt_bytes, public_inputs, &vk_type)
        }
        ProofData::Attestation { .. } => {
            err!(SolaaError::UnknownProofType)
        }
    }
}
