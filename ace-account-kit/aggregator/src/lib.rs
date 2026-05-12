//! AR-ACE Aggregator Service
//!
//! Off-chain service that collects attested transactions from relay nodes,
//! batches them, generates mock STARK receipts, and exposes an HTTP API.

pub mod batch;
pub mod collector;
pub mod error;
pub mod server;
pub mod types;
