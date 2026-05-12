//! ACE Layer API Server
//!
//! Usage: ace-api
//!
//! Endpoints:
//!   GET    /health                  Health check
//!   POST   /api/migrate             SA-Migration (mnemonic → id_com)
//!   GET    /api/account/:id_com     Query smart account state
//!   POST   /api/account/create      Create smart account
//!   POST   /api/prove/zk-ace        Generate ZK-ACE proof (demo mode)
//!   POST   /api/prove/ownership     Generate ZK-Ownership proof
//!   POST   /api/rotate              Key rotation
//!   GET    /api/stats               System stats

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("ACE_API_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3080);

    let app = ace_api::routes::build_router()
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("ACE Layer API listening on {}", addr);
    tracing::info!("  GET  /health                — health check");
    tracing::info!("  POST /api/migrate           — SA-migration");
    tracing::info!("  GET  /api/account/:id_com   — account state");
    tracing::info!("  POST /api/account/create    — create account");
    tracing::info!("  POST /api/prove/zk-ace      — ZK-ACE proof");
    tracing::info!("  POST /api/prove/ownership   — ownership proof");
    tracing::info!("  POST /api/rotate            — key rotation");
    tracing::info!("  GET  /api/stats             — system stats");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
