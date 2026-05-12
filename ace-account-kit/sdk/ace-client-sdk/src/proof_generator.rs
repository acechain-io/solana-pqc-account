//! ZK-ACE proof generation using Stwo Circle STARK (post-quantum secure).
//!
//! ## Primary Flow: Stwo STARK
//!
//! 1. Unseal the Rev32 artifact → get `rev`, `salt`
//! 2. Compute public inputs: `StwoEngine::compute_public_inputs`
//! 3. Generate STARK proof: `StwoEngine::prove` → `Vec<u8>` (bincode)
//! 4. Wrap in ZkAce receipt envelope: `build_stark_receipt`
//! 5. Submit `ProofData::Stark { receipt_bytes }` to on-chain `execute` instruction
//!
//! ## Alternative Flow: AR-ACE Attestation (Proof-Off-Path)
//!
//! For lower latency (proof verification deferred to aggregator):
//! 1. Generate Stwo proof (same as above)
//! 2. Forward to relay/aggregator → receives Ed25519 attestation
//! 3. Submit `execute_attested` with attestation → immediate on-chain effect

use ed25519_dalek::{SigningKey, Signer, Signature};
use zk_ace::{ReplayMode, StwoEngine, Witness, ZkAceEngine};
use crate::error::Result;

/// On-chain ProofData tag values (must match programs/ace-account-kit/src/verifier/types.rs).
pub const PROOF_TAG_STARK: u8 = 1;
pub const PROOF_TAG_ATTESTATION: u8 = 2;

// ---------------------------------------------------------------------------
// ZK-ACE STARK Circuit IDs (must match programs/ace-account-kit/src/vk.rs)
// ---------------------------------------------------------------------------

pub const ZK_ACE_CIRCUIT_ID: [u8; 32] = [
    0xcf, 0x15, 0xa0, 0xe5, 0xb4, 0xb3, 0xa0, 0xb3,
    0x27, 0xf7, 0xfc, 0xc6, 0x6b, 0x21, 0x39, 0x63,
    0x88, 0x75, 0xae, 0x15, 0x77, 0xa9, 0xd9, 0x73,
    0x5e, 0xe6, 0x53, 0x58, 0xb1, 0x0b, 0x7b, 0xe8,
];

pub const ZK_OWNERSHIP_CIRCUIT_ID: [u8; 32] = [
    0x00, 0x57, 0xe0, 0x43, 0x10, 0x70, 0x52, 0xc5,
    0xe9, 0xc2, 0xb0, 0xbf, 0xa5, 0x6b, 0x88, 0xad,
    0x56, 0x0c, 0x80, 0x0f, 0x04, 0xd2, 0x6e, 0x64,
    0x1c, 0x5b, 0x7b, 0xde, 0x5b, 0xcd, 0x45, 0xae,
];

// ---------------------------------------------------------------------------
// STARK proof generation
// ---------------------------------------------------------------------------

/// Result of ZK-ACE proof generation.
pub struct SolaaProofResult {
    /// Raw Stwo proof bytes (output of StwoEngine::prove).
    pub proof_bytes: Vec<u8>,
    /// Public inputs ready for on-chain submission.
    pub public_inputs: zk_ace::PublicInputs,
    /// Nonce used in the proof (for on-chain nonce verification).
    pub nonce: u64,
}

/// Generate a ZK-ACE authorization proof using Stwo Circle STARK.
///
/// # Arguments
/// * `rev`      — 32-byte REV (from unsealed Rev32 artifact)
/// * `salt`     — 32-byte salt (from sealed artifact)
/// * `domain`   — chain domain tag (Solana = 1)
/// * `tx_hash`  — SHA-256 of the transaction payload being authorized
/// * `nonce`    — monotonic nonce from the on-chain smart account
pub fn generate_ace_proof(
    rev: &[u8; 32],
    salt: &[u8; 32],
    domain: u64,
    tx_hash: &[u8; 32],
    nonce: u64,
) -> Result<SolaaProofResult> {
    let witness = Witness {
        rev: *rev,
        salt: *salt,
        alg_id: 0,
        domain,
        index: 0,
        nonce,
    };

    let mode = ReplayMode::NonceRegistry;

    let public_inputs = StwoEngine::compute_public_inputs(&witness, tx_hash, domain, mode)
        .map_err(|e| crate::error::SolaaClientError::ProofGeneration(e.to_string()))?;

    let proof_bytes = StwoEngine::prove(&witness, &public_inputs, mode)
        .map_err(|e| crate::error::SolaaClientError::ProofGeneration(e.to_string()))?;

    Ok(SolaaProofResult { proof_bytes, public_inputs, nonce })
}

/// Verify a Stwo STARK proof off-chain (e.g., for testing or relay validation).
pub fn verify_ace_proof(
    proof_bytes: &[u8],
    public_inputs: &zk_ace::PublicInputs,
    _domain: u64,
) -> Result<bool> {
    let mode = ReplayMode::NonceRegistry;
    StwoEngine::verify(proof_bytes, public_inputs, mode)
        .map_err(|e| crate::error::SolaaClientError::ProofGeneration(e.to_string()))
}

// ---------------------------------------------------------------------------
// On-chain serialization
// ---------------------------------------------------------------------------

/// Serialized proof data ready for on-chain submission.
pub struct SerializedProofData {
    pub bytes: Vec<u8>,
}

/// Build and serialize a STARK receipt envelope for on-chain `execute`.
///
/// The envelope format (matches stark.rs `ZkAceStarkReceipt::from_bytes`):
///   [ circuit_id(32) | num_inputs(1) | inputs(N×32) | proof_len(4) | proof_bytes ]
pub fn build_and_serialize_stark_receipt(
    result: &SolaaProofResult,
) -> SerializedProofData {
    let pi = &result.public_inputs;

    // Encode public inputs as 5 × [u8; 32] (little-endian Fr, as stored in PublicInputs)
    let committed_inputs: Vec<[u8; 32]> = vec![
        pi.id_com,
        pi.tx_hash,
        u64_to_fr_bytes(pi.domain),
        pi.target,
        pi.rp_com,
    ];

    serialize_receipt_bytes(&ZK_ACE_CIRCUIT_ID, &committed_inputs, &result.proof_bytes)
}

/// Serialize a receipt for the ZK-Ownership circuit.
pub fn build_and_serialize_ownership_receipt(
    proof_bytes: &[u8],
    public_inputs: &[[u8; 32]; 4],
) -> SerializedProofData {
    serialize_receipt_bytes(&ZK_OWNERSHIP_CIRCUIT_ID, public_inputs, proof_bytes)
}

fn serialize_receipt_bytes(
    circuit_id: &[u8; 32],
    inputs: &[[u8; 32]],
    proof_bytes: &[u8],
) -> SerializedProofData {
    let num_inputs = inputs.len();
    let proof_len = proof_bytes.len();
    let total = 1 + 32 + 1 + num_inputs * 32 + 4 + proof_len;
    let mut bytes = Vec::with_capacity(total);

    // ProofData enum tag: Stark = 1
    bytes.push(PROOF_TAG_STARK);
    bytes.extend_from_slice(circuit_id);
    bytes.push(num_inputs as u8);
    for input in inputs {
        bytes.extend_from_slice(input);
    }
    bytes.extend_from_slice(&(proof_len as u32).to_le_bytes());
    bytes.extend_from_slice(proof_bytes);

    SerializedProofData { bytes }
}

/// Encode a u64 domain as a 32-byte field element (big-endian in last 8 bytes).
/// Matches domain_to_field() in programs/ace-account-kit/src/verifier/public_inputs.rs.
fn u64_to_fr_bytes(v: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[24..32].copy_from_slice(&v.to_be_bytes());
    out
}

// ---------------------------------------------------------------------------
// AR-ACE: Ed25519 relay attestation (proof-off-path)
// ---------------------------------------------------------------------------

/// Attestation data for AR-ACE proof-off-path relay.
pub struct AttestationData {
    pub signature: [u8; 64],
    pub relay_pubkey: [u8; 32],
    pub obj_hash: [u8; 32],
    pub domain: u64,
    pub nonce: u64,
}

/// Generate an Ed25519 relay attestation.
///
/// The relay node signs: obj_hash(32) || domain(8 LE) || nonce(8 LE)
/// This matches the on-chain `build_attestation_message` format.
pub fn generate_attestation(
    signing_key: &SigningKey,
    obj_hash: &[u8; 32],
    domain: u64,
    nonce: u64,
) -> AttestationData {
    let mut message = Vec::with_capacity(48);
    message.extend_from_slice(obj_hash);
    message.extend_from_slice(&domain.to_le_bytes());
    message.extend_from_slice(&nonce.to_le_bytes());

    let signature: Signature = signing_key.sign(&message);

    AttestationData {
        signature: signature.to_bytes(),
        relay_pubkey: signing_key.verifying_key().to_bytes(),
        obj_hash: *obj_hash,
        domain,
        nonce,
    }
}

/// Serialize an attestation into on-chain ProofData format.
pub fn serialize_attestation(attest: &AttestationData) -> SerializedProofData {
    let mut bytes = Vec::with_capacity(1 + 64 + 32 + 32 + 8 + 8);
    bytes.push(PROOF_TAG_ATTESTATION);
    bytes.extend_from_slice(&attest.signature);
    bytes.extend_from_slice(&attest.relay_pubkey);
    bytes.extend_from_slice(&attest.obj_hash);
    bytes.extend_from_slice(&attest.domain.to_le_bytes());
    bytes.extend_from_slice(&attest.nonce.to_le_bytes());
    SerializedProofData { bytes }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_stark_proof() {
        let rev = [1u8; 32];
        let salt = [2u8; 32];
        let tx_hash = [3u8; 32];

        let result = generate_ace_proof(&rev, &salt, 1, &tx_hash, 0).unwrap();

        assert!(!result.proof_bytes.is_empty(), "proof bytes should not be empty");
        assert_ne!(result.public_inputs.id_com, [0u8; 32], "id_com should be non-zero");
        assert_eq!(result.nonce, 0, "nonce should match");

        let valid = verify_ace_proof(&result.proof_bytes, &result.public_inputs, 1).unwrap();
        assert!(valid, "valid proof should verify");
    }

    #[test]
    fn test_different_nonce_different_rp_com() {
        let rev = [1u8; 32];
        let salt = [2u8; 32];
        let tx_hash = [3u8; 32];

        let r1 = generate_ace_proof(&rev, &salt, 1, &tx_hash, 100).unwrap();
        let r2 = generate_ace_proof(&rev, &salt, 1, &tx_hash, 101).unwrap();

        assert_eq!(r1.public_inputs.id_com, r2.public_inputs.id_com, "same identity");
        assert_ne!(r1.public_inputs.rp_com, r2.public_inputs.rp_com, "different nonce → different rp_com");
    }

    #[test]
    fn test_build_and_serialize_stark_receipt() {
        let rev = [1u8; 32];
        let salt = [2u8; 32];
        let tx_hash = [3u8; 32];
        let result = generate_ace_proof(&rev, &salt, 1, &tx_hash, 0).unwrap();

        let serialized = build_and_serialize_stark_receipt(&result);
        assert_eq!(serialized.bytes[0], PROOF_TAG_STARK);

        // Verify circuit_id matches (bytes 1..33)
        assert_eq!(&serialized.bytes[1..33], &ZK_ACE_CIRCUIT_ID);

        // num_inputs = 5
        assert_eq!(serialized.bytes[33], 5);
    }

    #[test]
    fn test_attestation_format() {
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let obj_hash = [0xABu8; 32];
        let attest = generate_attestation(&signing_key, &obj_hash, 1, 42);

        let serialized = serialize_attestation(&attest);
        assert_eq!(serialized.bytes[0], PROOF_TAG_ATTESTATION);
        assert_eq!(serialized.bytes.len(), 1 + 64 + 32 + 32 + 8 + 8);
    }
}
