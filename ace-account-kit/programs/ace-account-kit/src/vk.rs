//! Circuit identifiers for the ZK-ACE STARK proof system (post-quantum).
//!
//! Circuit identity is established via `circuit_id` ([u8; 32]) which is the
//! SHA-256 of the ZK-ACE AIR constraint set description and Poseidon parameters.

// ---------------------------------------------------------------------------
// STARK Circuit IDs (Primary — Post-Quantum)
// ---------------------------------------------------------------------------

/// Circuit ID for ZK-ACE authorization circuit (Stwo, NonceRegistry mode).
///
/// Computed as: SHA-256("ZK-ACE-STWO-v1:NonceRegistry")
/// This identifies the exact constraint set. Any change to the AIR must update this.
pub fn get_zk_ace_circuit_id() -> [u8; 32] {
    [
        0xcf, 0x15, 0xa0, 0xe5, 0xb4, 0xb3, 0xa0, 0xb3,
        0x27, 0xf7, 0xfc, 0xc6, 0x6b, 0x21, 0x39, 0x63,
        0x88, 0x75, 0xae, 0x15, 0x77, 0xa9, 0xd9, 0x73,
        0x5e, 0xe6, 0x53, 0x58, 0xb1, 0x0b, 0x7b, 0xe8,
    ]
}

/// Circuit ID for ZK-Ownership cross-chain proof circuit (Stwo).
///
/// Computed as: SHA-256("ZK-OWNERSHIP-STWO-v1:Native")
pub fn get_zk_ownership_circuit_id() -> [u8; 32] {
    [
        0x00, 0x57, 0xe0, 0x43, 0x10, 0x70, 0x52, 0xc5,
        0xe9, 0xc2, 0xb0, 0xbf, 0xa5, 0x6b, 0x88, 0xad,
        0x56, 0x0c, 0x80, 0x0f, 0x04, 0xd2, 0x6e, 0x64,
        0x1c, 0x5b, 0x7b, 0xde, 0x5b, 0xcd, 0x45, 0xae,
    ]
}

// ---------------------------------------------------------------------------
// RISC Zero Image IDs (Level 2 Aggregation)
// ---------------------------------------------------------------------------

/// RISC Zero image ID for the ZK-ACE aggregated batch verifier.
///
/// Set this to the actual image ID after building:
///   cd circuits/zk-ace-risc0-guest && cargo risczero build
pub fn get_risc0_zk_ace_image_id() -> [u8; 32] {
    // Placeholder — set to actual RISC Zero image ID after building the guest.
    [0u8; 32]
}

/// RISC Zero image ID for ZK-Ownership aggregated verifier.
pub fn get_risc0_zk_ownership_image_id() -> [u8; 32] {
    [0u8; 32]
}

/// RISC Zero image ID for the batch STARK aggregator.
pub fn get_risc0_aggregator_image_id() -> [u8; 32] {
    [0u8; 32]
}
