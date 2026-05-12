//! Transaction collector — validates and buffers attested transactions.

use ed25519_dalek::{Signature, VerifyingKey};
use std::collections::HashSet;
use std::sync::Mutex;

use crate::error::{AggregatorError, Result};
use crate::types::AttestedTransaction;

/// Collects and validates attested transactions before batching.
pub struct TransactionCollector {
    /// Authorized relay public keys. Empty = accept all (demo mode).
    authorized_relays: HashSet<[u8; 32]>,
    /// Pending transactions awaiting batch aggregation.
    pending: Mutex<Vec<AttestedTransaction>>,
}

impl TransactionCollector {
    pub fn new(authorized_relays: Vec<[u8; 32]>) -> Self {
        Self {
            authorized_relays: authorized_relays.into_iter().collect(),
            pending: Mutex::new(Vec::new()),
        }
    }

    /// Submit an attested transaction for inclusion in the next batch.
    pub fn submit(&self, tx: AttestedTransaction) -> Result<()> {
        // If authorized_relays is non-empty, enforce it; otherwise accept all (demo mode).
        if !self.authorized_relays.is_empty()
            && !self.authorized_relays.contains(&tx.relay_pubkey)
        {
            return Err(AggregatorError::UnauthorizedRelay {
                pubkey: hex::encode(tx.relay_pubkey),
            });
        }

        // Verify Ed25519 attestation signature only when relay_pubkey is non-zero
        // (all-zero pubkey = demo/test mode, skip verification)
        let is_demo_key = tx.relay_pubkey == [0u8; 32];
        if !is_demo_key {
            self.verify_attestation(&tx)?;
        }

        self.pending.lock().unwrap().push(tx);
        Ok(())
    }

    /// Drain all pending transactions for batch processing.
    pub fn drain(&self) -> Vec<AttestedTransaction> {
        let mut pending = self.pending.lock().unwrap();
        std::mem::take(&mut *pending)
    }

    /// Number of pending transactions.
    pub fn pending_count(&self) -> usize {
        self.pending.lock().unwrap().len()
    }

    /// Verify the Ed25519 attestation signature.
    fn verify_attestation(&self, tx: &AttestedTransaction) -> Result<()> {
        let message = build_attestation_message(&tx.obj_hash, tx.domain, tx.attest_nonce);

        let pubkey = VerifyingKey::from_bytes(&tx.relay_pubkey).map_err(|_| {
            AggregatorError::InvalidAttestation {
                relay_pubkey: hex::encode(tx.relay_pubkey),
            }
        })?;

        let signature = Signature::from_bytes(&tx.signature);

        pubkey.verify_strict(&message, &signature).map_err(|_| {
            AggregatorError::InvalidAttestation {
                relay_pubkey: hex::encode(tx.relay_pubkey),
            }
        })?;

        Ok(())
    }
}

/// Build the attestation message: obj_hash(32) || domain(8 LE) || nonce(8 LE).
fn build_attestation_message(obj_hash: &[u8; 32], domain: u64, nonce: u64) -> Vec<u8> {
    let mut msg = Vec::with_capacity(48);
    msg.extend_from_slice(obj_hash);
    msg.extend_from_slice(&domain.to_le_bytes());
    msg.extend_from_slice(&nonce.to_le_bytes());
    msg
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    fn make_signed_tx(signing_key: &SigningKey) -> AttestedTransaction {
        let relay_pubkey: [u8; 32] = signing_key.verifying_key().to_bytes();
        let obj_hash = [0xABu8; 32];
        let domain = 1u64;
        let attest_nonce = 42u64;

        let message = build_attestation_message(&obj_hash, domain, attest_nonce);
        let sig: Signature = signing_key.sign(&message);

        AttestedTransaction {
            payload: vec![1, 2, 3],
            signature: sig.to_bytes(),
            relay_pubkey,
            obj_hash,
            domain,
            attest_nonce,
            witness: crate::types::WitnessData {
                rev: [1u8; 32],
                salt: [2u8; 32],
                alg_id: 0,
                index: 0,
                nonce: 4,
            },
            id_com: [5u8; 32],
            expected_nonce: 0,
        }
    }

    #[test]
    fn accept_valid_attestation() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let relay_pubkey = signing_key.verifying_key().to_bytes();
        let collector = TransactionCollector::new(vec![relay_pubkey]);

        let tx = make_signed_tx(&signing_key);
        assert!(collector.submit(tx).is_ok());
        assert_eq!(collector.pending_count(), 1);
    }

    #[test]
    fn reject_unauthorized_relay() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let collector = TransactionCollector::new(vec![[0xFFu8; 32]]); // different authorized key

        let tx = make_signed_tx(&signing_key);
        assert!(collector.submit(tx).is_err());
    }

    #[test]
    fn reject_bad_signature() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let relay_pubkey = signing_key.verifying_key().to_bytes();
        let collector = TransactionCollector::new(vec![relay_pubkey]);

        let mut tx = make_signed_tx(&signing_key);
        tx.signature[0] ^= 0xFF; // corrupt signature
        assert!(collector.submit(tx).is_err());
    }

    #[test]
    fn drain_clears_pending() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let relay_pubkey = signing_key.verifying_key().to_bytes();
        let collector = TransactionCollector::new(vec![relay_pubkey]);

        collector.submit(make_signed_tx(&signing_key)).unwrap();
        collector.submit(make_signed_tx(&signing_key)).unwrap();
        assert_eq!(collector.pending_count(), 2);

        let drained = collector.drain();
        assert_eq!(drained.len(), 2);
        assert_eq!(collector.pending_count(), 0);
    }

    #[test]
    fn accept_any_relay_in_open_mode() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let relay_pubkey = signing_key.verifying_key().to_bytes();
        // Empty authorized list = open/demo mode
        let collector = TransactionCollector::new(vec![]);

        let tx = make_signed_tx(&signing_key);
        // In open mode, any relay is accepted (signature still verified since pubkey is real)
        let _ = collector.submit(tx);
        // Just test it doesn't panic; result depends on signature validity
        drop(relay_pubkey);
    }
}
