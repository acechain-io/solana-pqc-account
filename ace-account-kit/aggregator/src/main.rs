//! AR-ACE Aggregator Service entry point.
//!
//! Usage: ace-aggregator
//!
//! Endpoints:
//!   POST /submit | /attest  — Submit attested transaction
//!   POST /flush             — Force batch processing
//!   GET  /pending           — Pending count
//!   GET  /batch/:id         — Batch receipt
//!   GET  /health            — Health check
//!   GET  /status            — Service status

use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("AGGREGATOR_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3030);

    // In production, load authorized relays from config or on-chain state.
    // Empty = open/demo mode (accept all relays).
    let authorized_relays: Vec<[u8; 32]> = Vec::new();

    let app = ace_aggregator::server::build_router(authorized_relays);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("AR-ACE Aggregator listening on {}", addr);
    tracing::info!("  POST /submit | /attest  — submit attestation");
    tracing::info!("  POST /flush             — force batch");
    tracing::info!("  GET  /pending           — pending count");
    tracing::info!("  GET  /batch/:id         — batch receipt");
    tracing::info!("  GET  /status            — service status");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
