use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolaaClientError {
    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    #[error("Invalid entropy length: expected {expected}, got {got}")]
    InvalidEntropyLength { expected: usize, got: usize },

    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Proof generation failed: {0}")]
    ProofGeneration(String),

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("Transaction build error: {0}")]
    TransactionBuild(String),

    #[error("Invalid REV32 format: {0}")]
    InvalidRev32(String),
}

pub type Result<T> = std::result::Result<T, SolaaClientError>;
