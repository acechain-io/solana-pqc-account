use anchor_lang::prelude::*;

#[error_code]
pub enum SolaaError {
    #[msg("ZK proof verification failed")]
    ProofVerificationFailed,

    #[msg("Invalid public inputs")]
    InvalidPublicInputs,

    #[msg("Identity commitment mismatch")]
    IdComMismatch,

    #[msg("Nonce mismatch — possible replay")]
    NonceMismatch,

    #[msg("Domain mismatch")]
    DomainMismatch,

    #[msg("No guardian set for this account")]
    NoGuardian,

    #[msg("Unauthorized: signer is not the guardian")]
    UnauthorizedGuardian,

    #[msg("Recovery already in progress")]
    RecoveryAlreadyPending,

    #[msg("No pending recovery to finalize")]
    NoPendingRecovery,

    #[msg("Recovery timelock has not elapsed")]
    TimelockNotElapsed,

    #[msg("Payload hash does not match public input")]
    PayloadHashMismatch,

    #[msg("Invalid proof format")]
    InvalidProofFormat,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("STARK proof verification failed")]
    StarkVerificationFailed,

    #[msg("Invalid attestation signature")]
    InvalidAttestation,

    #[msg("Relay node not authorized")]
    UnauthorizedRelay,

    #[msg("Unknown proof type for this context")]
    UnknownProofType,

    #[msg("Relay registry is full")]
    RelayRegistryFull,

    #[msg("Aggregated batch is empty")]
    EmptyBatch,

    #[msg("Batch state root mismatch")]
    BatchRootMismatch,

    #[msg("Foreign address or chain in args does not match the verified proof")]
    ForeignAddressMismatch,
}
