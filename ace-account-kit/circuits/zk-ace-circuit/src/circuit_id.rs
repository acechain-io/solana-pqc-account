//! Circuit ID computation for ZK-ACE and ZK-Ownership constraint sets.
//!
//! Since Stwo uses transparent setup, circuit identity is established by
//! the circuit label string (must match programs/ace-account-kit/src/vk.rs).

use sha2::{Digest, Sha256};

/// Compute SHA-256(label) as a 32-byte circuit identifier.
pub fn compute_circuit_id(label: &str) -> [u8; 32] {
    let hash = Sha256::digest(label.as_bytes());
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    out
}

/// ZK-ACE circuit ID: SHA-256("ZK-ACE-STWO-v1:NonceRegistry")
pub const ZK_ACE_CIRCUIT_ID: [u8; 32] = [
    0xcf, 0x15, 0xa0, 0xe5, 0xb4, 0xb3, 0xa0, 0xb3,
    0x27, 0xf7, 0xfc, 0xc6, 0x6b, 0x21, 0x39, 0x63,
    0x88, 0x75, 0xae, 0x15, 0x77, 0xa9, 0xd9, 0x73,
    0x5e, 0xe6, 0x53, 0x58, 0xb1, 0x0b, 0x7b, 0xe8,
];

/// ZK-Ownership circuit ID: SHA-256("ZK-OWNERSHIP-STWO-v1:Native")
pub const ZK_OWNERSHIP_CIRCUIT_ID: [u8; 32] = [
    0x00, 0x57, 0xe0, 0x43, 0x10, 0x70, 0x52, 0xc5,
    0xe9, 0xc2, 0xb0, 0xbf, 0xa5, 0x6b, 0x88, 0xad,
    0x56, 0x0c, 0x80, 0x0f, 0x04, 0xd2, 0x6e, 0x64,
    0x1c, 0x5b, 0x7b, 0xde, 0x5b, 0xcd, 0x45, 0xae,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zk_ace_circuit_id_matches_label() {
        let computed = compute_circuit_id("ZK-ACE-STWO-v1:NonceRegistry");
        assert_eq!(computed, ZK_ACE_CIRCUIT_ID,
            "ZK_ACE_CIRCUIT_ID constant does not match SHA-256 of label");
    }

    #[test]
    fn zk_ownership_circuit_id_matches_label() {
        let computed = compute_circuit_id("ZK-OWNERSHIP-STWO-v1:Native");
        assert_eq!(computed, ZK_OWNERSHIP_CIRCUIT_ID,
            "ZK_OWNERSHIP_CIRCUIT_ID constant does not match SHA-256 of label");
    }
}
