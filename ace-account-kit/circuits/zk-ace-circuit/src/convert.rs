//! Conversion between arkworks and Solana alt_bn128 encoding.
//!
//! Solana alt_bn128 follows Ethereum EIP-197:
//!   G1: (x_BE: 32, y_BE: 32)
//!   G2: (x.c1_BE: 32, x.c0_BE: 32, y.c1_BE: 32, y.c0_BE: 32)
//!
//! Arkworks uncompressed:
//!   G1: (x_LE: 32, y_LE: 32)
//!   G2: (x.c0_LE: 32, x.c1_LE: 32, y.c0_LE: 32, y.c1_LE: 32)

use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;

/// Convert a G1Affine point to Solana alt_bn128 format (64 bytes, big-endian).
pub fn g1_to_solana(p: &G1Affine) -> [u8; 64] {
    let mut out = [0u8; 64];
    // x coordinate: from ark LE to BE
    let x_bytes = p.x.into_bigint().to_bytes_le();
    out[0..32].copy_from_slice(&pad32(&x_bytes));
    out[0..32].reverse();
    // y coordinate: from ark LE to BE
    let y_bytes = p.y.into_bigint().to_bytes_le();
    out[32..64].copy_from_slice(&pad32(&y_bytes));
    out[32..64].reverse();
    out
}

/// Convert a G2Affine point to Solana alt_bn128 format (128 bytes, EIP-197 encoding).
///
/// EIP-197 G2 order: (x.c1_BE, x.c0_BE, y.c1_BE, y.c0_BE)
pub fn g2_to_solana(p: &G2Affine) -> [u8; 128] {
    let mut out = [0u8; 128];

    let xc0 = p.x.c0.into_bigint().to_bytes_le();
    let xc1 = p.x.c1.into_bigint().to_bytes_le();
    let yc0 = p.y.c0.into_bigint().to_bytes_le();
    let yc1 = p.y.c1.into_bigint().to_bytes_le();

    // EIP-197: c1 before c0
    out[0..32].copy_from_slice(&pad32(&xc1));
    out[0..32].reverse();
    out[32..64].copy_from_slice(&pad32(&xc0));
    out[32..64].reverse();
    out[64..96].copy_from_slice(&pad32(&yc1));
    out[64..96].reverse();
    out[96..128].copy_from_slice(&pad32(&yc0));
    out[96..128].reverse();

    out
}

/// Convert a BN254 Fr field element to 32-byte big-endian (as required for public inputs).
pub fn fr_to_solana(f: &Fr) -> [u8; 32] {
    let le = f.into_bigint().to_bytes_le();
    let mut be = pad32(&le);
    be.reverse();
    be
}

/// Pad or truncate a byte slice to exactly 32 bytes (zero-padded on right).
fn pad32(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let len = bytes.len().min(32);
    out[..len].copy_from_slice(&bytes[..len]);
    out
}

/// Convert a compressed Groth16 proof (128 bytes from arkworks) to Solana uncompressed format.
///
/// Arkworks compressed: A_compressed(32) + B_compressed(64) + C_compressed(32) = 128 bytes
/// Solana uncompressed: A_uncompressed(64) + B_uncompressed(128) + C_uncompressed(64) = 256 bytes
///
/// The on-chain program struct Groth16Proof { a: [u8;64], b: [u8;128], c: [u8;64] }
pub fn groth16_proof_compressed_to_solana(
    compressed: &[u8],
) -> Result<([u8; 64], [u8; 128], [u8; 64]), String> {
    use ark_bn254::Bn254;
    use ark_groth16::Proof;
    use ark_serialize::CanonicalDeserialize;

    let proof = Proof::<Bn254>::deserialize_compressed(compressed)
        .map_err(|e| format!("failed to deserialize proof: {e}"))?;

    let a = g1_to_solana(&proof.a);
    let b = g2_to_solana(&proof.b);
    let c = g1_to_solana(&proof.c);

    Ok((a, b, c))
}

/// Convert PublicInputs to 5 × [u8; 32] field elements in Solana big-endian format.
///
/// Returns [id_com, tx_hash, domain_as_fr, target, rp_com]
pub fn public_inputs_to_solana_fields(pi: &zk_ace::PublicInputs) -> [[u8; 32]; 5] {
    use ark_ff::PrimeField;

    fn u32_arr_to_be(bytes: &[u8; 32]) -> [u8; 32] {
        // pi fields are already stored as LE bytes from Fr; convert to BE for Solana
        let mut be = *bytes;
        be.reverse();
        be
    }

    // domain is a u64; convert to Fr then to BE bytes
    let domain_fr = ark_bn254::Fr::from(pi.domain);
    let domain_le = domain_fr.into_bigint().to_bytes_le();
    let mut domain_be = pad32(&domain_le);
    domain_be.reverse();

    [
        u32_arr_to_be(&pi.id_com),
        u32_arr_to_be(&pi.tx_hash),
        domain_be,
        u32_arr_to_be(&pi.target),
        u32_arr_to_be(&pi.rp_com),
    ]
}
