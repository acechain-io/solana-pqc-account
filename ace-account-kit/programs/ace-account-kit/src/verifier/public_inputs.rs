//! Public input parsing for ZK-ACE proofs.
//!
//! ZK-ACE has 5 public inputs (each a BN254 scalar field element, 32 bytes):
//!   0: id_com    — identity commitment
//!   1: tx_hash   — hash of the transaction payload
//!   2: domain    — chain domain tag
//!   3: target    — derivation target
//!   4: rp_com    — replay-prevention commitment (= Poseidon(nonce))

use anchor_lang::prelude::*;
use crate::errors::SolaaError;

/// Number of public inputs for ZK-ACE authorization proof.
pub const NUM_PUBLIC_INPUTS: usize = 5;

/// Number of public inputs for ZK-Ownership cross-chain proof.
pub const NUM_OWNERSHIP_INPUTS: usize = 4;

/// Parsed public inputs for a ZK-ACE authorization proof.
#[derive(Debug, Clone)]
pub struct ZkAcePublicInputs {
    pub id_com: [u8; 32],
    pub tx_hash: [u8; 32],
    pub domain: [u8; 32],
    pub target: [u8; 32],
    pub rp_com: [u8; 32],
}

impl ZkAcePublicInputs {
    /// Parse from raw bytes (5 × 32 = 160 bytes).
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        require!(data.len() == NUM_PUBLIC_INPUTS * 32, SolaaError::InvalidPublicInputs);

        let mut id_com = [0u8; 32];
        let mut tx_hash = [0u8; 32];
        let mut domain = [0u8; 32];
        let mut target = [0u8; 32];
        let mut rp_com = [0u8; 32];

        id_com.copy_from_slice(&data[0..32]);
        tx_hash.copy_from_slice(&data[32..64]);
        domain.copy_from_slice(&data[64..96]);
        target.copy_from_slice(&data[96..128]);
        rp_com.copy_from_slice(&data[128..160]);

        Ok(Self { id_com, tx_hash, domain, target, rp_com })
    }

    /// Return as an array of field element bytes for the verifier.
    pub fn as_field_elements(&self) -> [[u8; 32]; NUM_PUBLIC_INPUTS] {
        [self.id_com, self.tx_hash, self.domain, self.target, self.rp_com]
    }
}

/// Parsed public inputs for a ZK-Ownership proof.
#[derive(Debug, Clone)]
pub struct ZkOwnershipPublicInputs {
    pub id_com: [u8; 32],
    pub solana_address: [u8; 32],
    pub foreign_address: [u8; 32],
    pub foreign_chain: [u8; 32],
}

impl ZkOwnershipPublicInputs {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        require!(data.len() == NUM_OWNERSHIP_INPUTS * 32, SolaaError::InvalidPublicInputs);

        let mut id_com = [0u8; 32];
        let mut solana_address = [0u8; 32];
        let mut foreign_address = [0u8; 32];
        let mut foreign_chain = [0u8; 32];

        id_com.copy_from_slice(&data[0..32]);
        solana_address.copy_from_slice(&data[32..64]);
        foreign_address.copy_from_slice(&data[64..96]);
        foreign_chain.copy_from_slice(&data[96..128]);

        Ok(Self { id_com, solana_address, foreign_address, foreign_chain })
    }

    pub fn as_field_elements(&self) -> [[u8; 32]; NUM_OWNERSHIP_INPUTS] {
        [self.id_com, self.solana_address, self.foreign_address, self.foreign_chain]
    }
}

/// Compute a simple domain value as a 32-byte field element.
/// Domain tag is stored as u64, padded to 32 bytes (big-endian).
pub fn domain_to_field(domain: u64) -> [u8; 32] {
    let mut field = [0u8; 32];
    field[24..32].copy_from_slice(&domain.to_be_bytes());
    field
}

/// Compute a simple nonce value as a 32-byte field element.
pub fn nonce_to_field(nonce: u64) -> [u8; 32] {
    let mut field = [0u8; 32];
    field[24..32].copy_from_slice(&nonce.to_be_bytes());
    field
}
