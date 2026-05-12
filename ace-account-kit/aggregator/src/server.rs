//! HTTP server for the aggregator service.
//!
//! Endpoints:
//! - POST /submit        — Submit an attested transaction
//! - POST /attest        — Alias for /submit
//! - POST /flush         — Force batch processing of all pending transactions
//! - GET  /pending       — Number of pending attestations
//! - GET  /batch/:id     — Get batch receipt by id (in-memory)
//! - GET  /health        — Health check
//! - GET  /status        — Aggregator status

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::batch;
use crate::collector::TransactionCollector;
use crate::types::{AggregatorStatus, AttestedTransaction, BatchResult};

/// Shared application state.
pub struct AppState {
    pub collector: TransactionCollector,
    pub batches_produced: Mutex<u64>,
    pub total_txs_processed: Mutex<u64>,
    pub batch_store: Mutex<HashMap<u64, BatchResult>>,
}

/// Build the axum router.
pub fn build_router(authorized_relays: Vec<[u8; 32]>) -> Router {
    let state = Arc::new(AppState {
        collector: TransactionCollector::new(authorized_relays),
        batches_produced: Mutex::new(0),
        total_txs_processed: Mutex::new(0),
        batch_store: Mutex::new(HashMap::new()),
    });

    Router::new()
        .route("/submit", post(submit_tx))
        .route("/attest", post(submit_tx))
        .route("/flush", post(flush_batch))
        .route("/pending", get(get_pending))
        .route("/batch/{id}", get(get_batch))
        .route("/health", get(health))
        .route("/status", get(get_status))
        .with_state(state)
}

#[derive(Serialize)]
struct SubmitResponse {
    accepted: bool,
    pending_count: usize,
}

#[derive(Serialize)]
struct FlushResponse {
    success: bool,
    batch_id: Option<u64>,
    num_txs: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct PendingResponse {
    count: usize,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: "0.1.0",
    })
}

async fn submit_tx(
    State(state): State<Arc<AppState>>,
    Json(tx): Json<AttestedTransaction>,
) -> (StatusCode, Json<SubmitResponse>) {
    match state.collector.submit(tx) {
        Ok(()) => (
            StatusCode::OK,
            Json(SubmitResponse {
                accepted: true,
                pending_count: state.collector.pending_count(),
            }),
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(SubmitResponse {
                accepted: false,
                pending_count: state.collector.pending_count(),
            }),
        ),
    }
}

async fn flush_batch(State(state): State<Arc<AppState>>) -> (StatusCode, Json<FlushResponse>) {
    let transactions = state.collector.drain();
    if transactions.is_empty() {
        return (
            StatusCode::OK,
            Json(FlushResponse {
                success: false,
                batch_id: None,
                num_txs: 0,
                error: Some("No pending transactions".to_string()),
            }),
        );
    }

    let num_txs = transactions.len() as u64;
    match batch::process_batch(&transactions) {
        Ok(result) => {
            let batch_id = {
                let mut bp = state.batches_produced.lock().unwrap();
                *bp += 1;
                *bp
            };
            *state.total_txs_processed.lock().unwrap() += num_txs;
            state.batch_store.lock().unwrap().insert(batch_id, result);

            (
                StatusCode::OK,
                Json(FlushResponse {
                    success: true,
                    batch_id: Some(batch_id),
                    num_txs,
                    error: None,
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(FlushResponse {
                success: false,
                batch_id: None,
                num_txs: 0,
                error: Some(e.to_string()),
            }),
        ),
    }
}

async fn get_pending(State(state): State<Arc<AppState>>) -> Json<PendingResponse> {
    Json(PendingResponse {
        count: state.collector.pending_count(),
    })
}

async fn get_batch(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<BatchResult>, StatusCode> {
    state
        .batch_store
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn get_status(State(state): State<Arc<AppState>>) -> Json<AggregatorStatus> {
    Json(AggregatorStatus {
        pending_txs: state.collector.pending_count(),
        batches_produced: *state.batches_produced.lock().unwrap(),
        total_txs_processed: *state.total_txs_processed.lock().unwrap(),
    })
}
