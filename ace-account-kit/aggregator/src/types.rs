use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// An attested transaction submitted by a relay node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttestedTransaction {
    /// The payload to execute (opaque bytes for the on-chain program).
    pub payload: Vec<u8>,
    /// Ed25519 signature from the relay over the attestation message.
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
    /// Relay's Ed25519 public key.
    pub relay_pubkey: [u8; 32],
    /// Hash of the object being attested (tx content hash).
    pub obj_hash: [u8; 32],
    /// Domain identifier for the smart account.
    pub domain: u64,
    /// Attestation nonce for replay prevention.
    pub attest_nonce: u64,
    /// ZK-ACE witness components (encrypted or in clear for hackathon).
    pub witness: WitnessData,
    /// The account id_com this transaction targets.
    pub id_com: [u8; 32],
    /// Expected nonce of the account before this tx.
    pub expected_nonce: u64,
}

/// ZK-ACE witness data carried in the attested transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WitnessData {
    pub rev: [u8; 32],
    pub salt: [u8; 32],
    pub alg_id: u64,
    pub index: u64,
    pub nonce: u64,
}

/// Result of batch aggregation, ready for on-chain submission.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchResult {
    /// The mock STARK receipt bytes (batch_hash + metadata).
    pub receipt_bytes: Vec<u8>,
    /// Account updates to submit as remaining_accounts.
    pub updates: Vec<AccountUpdateEntry>,
    /// Number of transactions in this batch.
    pub num_txs: u64,
}

/// An account update entry for on-chain submission.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountUpdateEntry {
    pub id_com: [u8; 32],
    pub expected_nonce: u64,
    pub tx_count: u64,
}

/// Status of the aggregator service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AggregatorStatus {
    pub pending_txs: usize,
    pub batches_produced: u64,
    pub total_txs_processed: u64,
}
