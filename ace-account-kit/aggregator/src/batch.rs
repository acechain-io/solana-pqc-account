//! Batch processor — generates mock STARK receipt for demo.

use sha2::{Digest, Sha256};

use crate::error::{AggregatorError, Result};
use crate::types::{AccountUpdateEntry, AttestedTransaction, BatchResult};

/// Maximum transactions per batch.
pub const MAX_BATCH_SIZE: usize = 256;

/// Process a batch of attested transactions (mock mode for hackathon demo).
/// Computes batch_hash = SHA-256(sorted per-tx hashes) and returns a mock receipt.
pub fn process_batch(transactions: &[AttestedTransaction]) -> Result<BatchResult> {
    if transactions.is_empty() {
        return Err(AggregatorError::EmptyBatch);
    }
    if transactions.len() > MAX_BATCH_SIZE {
        return Err(AggregatorError::BatchTooLarge {
            size: transactions.len(),
            max: MAX_BATCH_SIZE,
        });
    }

    // Compute per-tx hashes: SHA-256(obj_hash || id_com || domain.le || nonce.le)
    let mut tx_hashes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|atx| {
            let mut h = Sha256::new();
            h.update(&atx.obj_hash);
            h.update(&atx.id_com);
            h.update(atx.domain.to_le_bytes());
            h.update(atx.attest_nonce.to_le_bytes());
            h.finalize().into()
        })
        .collect();

    // Sort for determinism
    tx_hashes.sort();

    // batch_hash = SHA-256(concat of sorted tx hashes)
    let mut batch_hasher = Sha256::new();
    for h in &tx_hashes {
        batch_hasher.update(h);
    }
    let batch_hash: [u8; 32] = batch_hasher.finalize().into();

    // Compute update_root: SHA-256(concat of id_com||expected_nonce_le||tx_count_le for each update)
    let updates_temp = build_update_entries(
        &transactions.iter().map(|atx| (atx.id_com, atx.expected_nonce)).collect::<Vec<_>>()
    );
    let mut update_data: Vec<u8> = Vec::new();
    for u in &updates_temp {
        update_data.extend_from_slice(&u.id_com);
        update_data.extend_from_slice(&u.expected_nonce.to_le_bytes());
        update_data.extend_from_slice(&u.tx_count.to_le_bytes());
    }
    let update_root: [u8; 32] = {
        let mut h = Sha256::new();
        h.update(&update_data);
        h.finalize().into()
    };

    // circuit_id matching vk::get_zk_ace_circuit_id()
    let circuit_id: [u8; 32] = [
        0xcf, 0x15, 0xa0, 0xe5, 0xb4, 0xb3, 0xa0, 0xb3,
        0x27, 0xf7, 0xfc, 0xc6, 0x6b, 0x21, 0x39, 0x63,
        0x88, 0x75, 0xae, 0x15, 0x77, 0xa9, 0xd9, 0x73,
        0x5e, 0xe6, 0x53, 0x58, 0xb1, 0x0b, 0x7b, 0xe8,
    ];

    // Mock proof bytes: batch_hash (signals "off-chain verified")
    let proof_bytes = batch_hash.to_vec();

    // Wire format: circuit_id(32)|batch_hash(32)|num_txs(8)|update_root(32)|proof_len(4)|proof
    let mut receipt_bytes = Vec::with_capacity(32 + 32 + 8 + 32 + 4 + proof_bytes.len());
    receipt_bytes.extend_from_slice(&circuit_id);
    receipt_bytes.extend_from_slice(&batch_hash);
    receipt_bytes.extend_from_slice(&(transactions.len() as u64).to_le_bytes());
    receipt_bytes.extend_from_slice(&update_root);
    receipt_bytes.extend_from_slice(&(proof_bytes.len() as u32).to_le_bytes());
    receipt_bytes.extend_from_slice(&proof_bytes);

    let id_coms: Vec<([u8; 32], u64)> = transactions
        .iter()
        .map(|atx| (atx.id_com, atx.expected_nonce))
        .collect();

    let updates = build_update_entries(&id_coms);

    Ok(BatchResult {
        receipt_bytes,
        updates,
        num_txs: transactions.len() as u64,
    })
}

fn build_update_entries(id_coms: &[([u8; 32], u64)]) -> Vec<AccountUpdateEntry> {
    let mut updates: Vec<AccountUpdateEntry> = Vec::new();
    for (id_com, expected_nonce) in id_coms {
        if let Some(existing) = updates.iter_mut().find(|u| u.id_com == *id_com) {
            existing.tx_count += 1;
        } else {
            updates.push(AccountUpdateEntry {
                id_com: *id_com,
                expected_nonce: *expected_nonce,
                tx_count: 1,
            });
        }
    }
    updates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::WitnessData;

    fn make_test_tx(domain: u64, nonce: u64) -> AttestedTransaction {
        AttestedTransaction {
            payload: vec![],
            signature: [0u8; 64],
            relay_pubkey: [0u8; 32],
            obj_hash: [0xAAu8; 32],
            domain,
            attest_nonce: nonce,
            witness: WitnessData {
                rev: [1u8; 32],
                salt: [2u8; 32],
                alg_id: 0,
                index: 0,
                nonce,
            },
            id_com: [3u8; 32],
            expected_nonce: 0,
        }
    }

    #[test]
    fn process_single_tx_batch() {
        let atx = make_test_tx(1, 0);
        let result = process_batch(&[atx]).unwrap();
        assert_eq!(result.num_txs, 1);
        assert_eq!(result.updates.len(), 1);
        assert_eq!(result.updates[0].tx_count, 1);
        assert!(result.receipt_bytes.len() >= 108);
    }

    #[test]
    fn reject_empty_batch() {
        assert!(process_batch(&[]).is_err());
    }

    #[test]
    fn batch_hash_deterministic() {
        let txs = vec![make_test_tx(1, 0), make_test_tx(1, 1)];
        let r1 = process_batch(&txs).unwrap();
        let r2 = process_batch(&txs).unwrap();
        assert_eq!(r1.receipt_bytes, r2.receipt_bytes);
    }
}
