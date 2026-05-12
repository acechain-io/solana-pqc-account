//! API route handlers.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Instant;

use crate::account_manager::{derive_pda, AccountManager};
use crate::models::*;

pub struct AppState {
    pub manager: AccountManager,
    pub start_time: Instant,
}

pub fn build_router() -> Router {
    let state = Arc::new(AppState {
        manager: AccountManager::new(),
        start_time: Instant::now(),
    });

    Router::new()
        .route("/health", get(health))
        .route("/api/migrate", post(migrate))
        .route("/api/account/:id_com", get(get_account))
        .route("/api/account/create", post(create_account))
        .route("/api/prove/zk-ace", post(prove_zk_ace))
        .route("/api/prove/ownership", post(prove_ownership))
        .route("/api/rotate", post(rotate))
        .route("/api/stats", get(stats))
        .with_state(state)
}

// ── Health ───────────────────────────────────────────────

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: "0.1.0",
    })
}

// ── Migration ────────────────────────────────────────────

async fn migrate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MigrateRequest>,
) -> Json<MigrateResponse> {
    // id_com = SHA-256(mnemonic bytes) — simplified for demo
    let mut h = Sha256::new();
    h.update(req.mnemonic.as_bytes());
    let id_com_bytes: [u8; 32] = h.finalize().into();
    let id_com = hex::encode(id_com_bytes);

    // address derived from id_com using the same PDA derivation as on-chain
    let address = derive_pda(&id_com_bytes, &state.manager.program_id);

    // artifact_encrypted = SHA-256(id_com || passphrase)
    let passphrase = req.passphrase.unwrap_or_default();
    let mut h2 = Sha256::new();
    h2.update(&id_com_bytes);
    h2.update(passphrase.as_bytes());
    let artifact: [u8; 32] = h2.finalize().into();
    let artifact_encrypted = hex::encode(artifact);

    Json(MigrateResponse {
        id_com,
        address,
        artifact_encrypted,
    })
}

// ── Account ──────────────────────────────────────────────

async fn get_account(
    State(state): State<Arc<AppState>>,
    Path(id_com): Path<String>,
) -> Result<Json<AccountState>, StatusCode> {
    state
        .manager
        .get_account(&id_com)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAccountRequest>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    let domain = req.domain.unwrap_or(1);
    let account = state
        .manager
        .create_account(req.id_com, domain, req.guardian);

    // Mock tx signature
    let mut h = Sha256::new();
    h.update(account.pda.as_bytes());
    let sig = hex::encode(&h.finalize()[..16]);

    (
        StatusCode::CREATED,
        Json(CreateAccountResponse {
            pda: account.pda,
            tx_signature: format!("mock_sig_{}", sig),
        }),
    )
}

// ── Proofs ───────────────────────────────────────────────

async fn prove_zk_ace(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ProveZkAceRequest>,
) -> Json<ProveZkAceResponse> {
    state.manager.increment_proofs();

    // Deterministic mock proof: SHA-256(id_com || tx_hash || domain || nonce) repeated
    let mut seed_hasher = Sha256::new();
    seed_hasher.update(req.id_com.as_bytes());
    seed_hasher.update(req.tx_hash.as_bytes());
    seed_hasher.update(req.domain.to_le_bytes());
    seed_hasher.update(req.nonce.to_le_bytes());
    let seed: [u8; 32] = seed_hasher.finalize().into();

    // proof = 128 bytes (4x sha256 rounds)
    let mut proof_bytes = Vec::with_capacity(128);
    for i in 0u8..4 {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(&[i]);
        proof_bytes.extend_from_slice(&h.finalize());
    }

    // public_inputs = 160 bytes (5x sha256 rounds)
    let mut pi_bytes = Vec::with_capacity(160);
    for i in 0u8..5 {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(&[0xA0u8 | i]);
        pi_bytes.extend_from_slice(&h.finalize());
    }

    Json(ProveZkAceResponse {
        proof: hex::encode(&proof_bytes),
        public_inputs: hex::encode(&pi_bytes),
        prove_time_ms: 52,
    })
}

async fn prove_ownership(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ProveOwnershipRequest>,
) -> Json<ProveOwnershipResponse> {
    state.manager.increment_proofs();

    let mut seed_hasher = Sha256::new();
    seed_hasher.update(req.id_com.as_bytes());
    seed_hasher.update(req.solana_address.as_bytes());
    seed_hasher.update(req.foreign_chain.as_bytes());
    seed_hasher.update(req.foreign_address.as_bytes());
    let seed: [u8; 32] = seed_hasher.finalize().into();

    // proof = 256 bytes
    let mut proof_bytes = Vec::with_capacity(256);
    for i in 0u8..8 {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(&[i]);
        proof_bytes.extend_from_slice(&h.finalize());
    }

    // public_inputs = 128 bytes
    let mut pi_bytes = Vec::with_capacity(128);
    for i in 0u8..4 {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(&[0xB0u8 | i]);
        pi_bytes.extend_from_slice(&h.finalize());
    }

    Json(ProveOwnershipResponse {
        proof: hex::encode(&proof_bytes),
        public_inputs: hex::encode(&pi_bytes),
        prove_time_ms: 1800,
    })
}

// ── Rotation ─────────────────────────────────────────────

async fn rotate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RotateRequest>,
) -> Result<Json<RotateResponse>, (StatusCode, Json<ErrorResponse>)> {
    let old = req.id_com.clone();
    let new = req.new_id_com.clone();

    // If the old account doesn't exist yet, still allow rotation (creates new)
    state.manager.rotate_account(&old, &new);

    let mut h = Sha256::new();
    h.update(new.as_bytes());
    let sig = hex::encode(&h.finalize()[..16]);

    Ok(Json(RotateResponse {
        tx_signature: format!("mock_sig_{}", sig),
        old_id_com: old,
        new_id_com: new,
    }))
}

// ── Stats ────────────────────────────────────────────────

async fn stats(State(state): State<Arc<AppState>>) -> Json<StatsResponse> {
    Json(StatsResponse {
        total_accounts: state.manager.total_accounts(),
        total_proofs: state.manager.total_proofs(),
        avg_prove_time_ms: 52,
        supported_chains: vec![
            "solana".to_string(),
            "ethereum".to_string(),
            "bitcoin".to_string(),
        ],
    })
}
