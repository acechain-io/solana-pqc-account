use thiserror::Error;

#[derive(Error, Debug)]
pub enum AggregatorError {
    #[error("Invalid attestation signature for relay {relay_pubkey}")]
    InvalidAttestation { relay_pubkey: String },

    #[error("Relay {pubkey} is not authorized")]
    UnauthorizedRelay { pubkey: String },

    #[error("Constraint verification failed for tx {tx_index}: {reason}")]
    ConstraintFailure { tx_index: usize, reason: String },

    #[error("Batch is empty")]
    EmptyBatch,

    #[error("Batch size {size} exceeds maximum {max}")]
    BatchTooLarge { size: usize, max: usize },

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, AggregatorError>;
