//! End-to-end Stwo STARK prove + verify test.
//!
//! Usage: cargo run --release --bin prove_test

use zk_ace::{ReplayMode, StwoEngine, Witness, ZkAceEngine};
use zk_ace_circuit::ZK_ACE_CIRCUIT_ID;

fn main() {
    let rev: [u8; 32] = { let mut r = [0u8; 32]; r[0] = 0x42; r[1] = 0x43; r };
    let salt: [u8; 32] = { let mut s = [0u8; 32]; s[0] = 0xAA; s };
    let tx_hash: [u8; 32] = { let mut t = [0u8; 32]; t[0] = 0xFF; t[1] = 0xAB; t };
    let domain: u64 = 1;
    let nonce: u64 = 7;

    let witness = Witness { rev, salt, alg_id: 0, domain, index: 0, nonce };

    eprintln!("[prove_test] Computing public inputs (Stwo, NonceRegistry)...");
    let pi = StwoEngine::compute_public_inputs(&witness, &tx_hash, domain, ReplayMode::NonceRegistry)
        .expect("compute_public_inputs failed");

    eprintln!("[prove_test] id_com  = {}", hex::encode(&pi.id_com));
    eprintln!("[prove_test] tx_hash = {}", hex::encode(&pi.tx_hash));

    eprintln!("[prove_test] Generating Stwo STARK proof (post-quantum)...");
    let proof_bytes = StwoEngine::prove(&witness, &pi, ReplayMode::NonceRegistry)
        .expect("prove failed");
    eprintln!("[prove_test] Proof size: {} bytes", proof_bytes.len());

    eprintln!("[prove_test] Verifying...");
    let valid = StwoEngine::verify(&proof_bytes, &pi, ReplayMode::NonceRegistry)
        .expect("verify failed");
    assert!(valid, "proof verification failed");
    eprintln!("[prove_test] ✓ Verification passed");

    // Build the on-chain receipt envelope
    let inputs: Vec<[u8; 32]> = vec![
        pi.id_com,
        pi.tx_hash,
        { let mut d = [0u8; 32]; d[..8].copy_from_slice(&domain.to_le_bytes()); d },
        pi.target,
        pi.rp_com,
    ];

    let mut receipt_bytes = Vec::new();
    receipt_bytes.extend_from_slice(&ZK_ACE_CIRCUIT_ID);
    receipt_bytes.push(inputs.len() as u8);
    for inp in &inputs { receipt_bytes.extend_from_slice(inp); }
    receipt_bytes.extend_from_slice(&(proof_bytes.len() as u32).to_le_bytes());
    receipt_bytes.extend_from_slice(&proof_bytes);

    println!();
    println!("// Stwo STARK proof receipt (hex) for on-chain submission:");
    println!("// Total receipt size: {} bytes", receipt_bytes.len());
    println!("let receipt_hex = \"{}\";", hex::encode(&receipt_bytes));

    eprintln!();
    eprintln!("[prove_test] Done. Use receipt_hex in TypeScript integration tests.");
}
