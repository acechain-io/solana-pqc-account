use serde::{Deserialize, Serialize};

// ── Migration ────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrateRequest {
    pub mnemonic: String,
    pub passphrase: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrateResponse {
    pub id_com: String,
    pub address: String,
    pub artifact_encrypted: String,
}

// ── Account ──────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub id_com: String,
    pub domain: Option<u64>,
    pub guardian: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub pda: String,
    pub tx_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub pda: String,
    pub nonce: u64,
    pub domain: u64,
    pub guardian: Option<String>,
    pub created_at: u64,
}

// ── Proofs ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveZkAceRequest {
    pub id_com: String,
    pub tx_hash: String,
    pub domain: u64,
    pub nonce: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveZkAceResponse {
    pub proof: String,
    pub public_inputs: String,
    pub prove_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveOwnershipRequest {
    pub id_com: String,
    pub solana_address: String,
    pub foreign_chain: String,
    pub foreign_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveOwnershipResponse {
    pub proof: String,
    pub public_inputs: String,
    pub prove_time_ms: u64,
}

// ── Rotation ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct RotateRequest {
    pub id_com: String,
    pub new_id_com: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotateResponse {
    pub tx_signature: String,
    pub old_id_com: String,
    pub new_id_com: String,
}

// ── Stats ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_accounts: u64,
    pub total_proofs: u64,
    pub avg_prove_time_ms: u64,
    pub supported_chains: Vec<String>,
}

// ── Health ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

// ── Error ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
