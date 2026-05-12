use anchor_lang::prelude::*;

pub mod errors;
pub mod verifier;
pub mod instructions;
pub mod state;
pub mod vk;

use instructions::*;

declare_id!("EgKrUBUsQjC7BZ7xJGNLkDPP5UnvQ1u9Ldx7uRThNmL5");

#[program]
pub mod solaa {
    use super::*;

    /// Create a new SolAA Smart Account (PDA).
    ///
    /// The account is seeded by `[b"solaa", id_com]`, so the PDA address
    /// is deterministic from the identity commitment.
    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        instructions::initialize::handler(ctx, args)
    }

    /// Execute a transaction authorized by a ZK proof (STARK).
    ///
    /// No Ed25519 signature required — authorization comes entirely from
    /// the ZK proof verified via the proof-system-agnostic verifier.
    pub fn execute(ctx: Context<Execute>, args: ExecuteArgs) -> Result<()> {
        instructions::execute::handler(ctx, args)
    }

    /// Execute a transaction via AR-ACE attestation (proof-off-path).
    ///
    /// Authorization comes from a relay attestation. Full ZK validity
    /// proof is deferred to the aggregator/builder.
    pub fn execute_attested(ctx: Context<ExecuteAttested>, args: ExecuteAttestedArgs) -> Result<()> {
        instructions::execute_attested::handler(ctx, args)
    }

    /// Rotate the identity commitment (key rotation / PQC upgrade).
    ///
    /// Proves ownership of the current key via ZK proof, then updates
    /// id_com to a new value. The PDA address does NOT change.
    pub fn rotate_key(ctx: Context<RotateKey>, args: RotateKeyArgs) -> Result<()> {
        instructions::rotate_key::handler(ctx, args)
    }

    /// Guardian initiates account recovery.
    ///
    /// Starts a timelock period during which the original owner can cancel.
    pub fn initiate_recovery(
        ctx: Context<InitiateRecovery>,
        args: InitiateRecoveryArgs,
    ) -> Result<()> {
        instructions::recovery::handler_initiate(ctx, args)
    }

    /// Finalize a pending recovery after the timelock has elapsed.
    pub fn finalize_recovery(ctx: Context<FinalizeRecovery>, args: FinalizeRecoveryArgs) -> Result<()> {
        instructions::recovery::handler_finalize(ctx, args)
    }

    /// Cancel a pending recovery (requires ZK proof of current ownership).
    pub fn cancel_recovery(
        ctx: Context<CancelRecovery>,
        args: CancelRecoveryArgs,
    ) -> Result<()> {
        instructions::recovery::handler_cancel(ctx, args)
    }

    /// Verify a ZK-Ownership cross-chain proof and store the result on-chain.
    pub fn verify_ownership(
        ctx: Context<VerifyOwnership>,
        args: VerifyOwnershipArgs,
    ) -> Result<()> {
        instructions::verify_ownership::handler(ctx, args)
    }

    /// One-time initialization of the relay registry (sets authority).
    /// Must be called before any relay can be registered.
    pub fn initialize_relay_registry(ctx: Context<InitializeRelayRegistry>) -> Result<()> {
        instructions::register_relay::initialize_relay_registry_handler(ctx)
    }

    /// Register a relay node for AR-ACE attestation (requires authority).
    pub fn register_relay(ctx: Context<RegisterRelay>, args: RegisterRelayArgs) -> Result<()> {
        instructions::register_relay::register_relay_handler(ctx, args)
    }

    /// Remove a relay node from the registry (requires authority).
    pub fn remove_relay(ctx: Context<RemoveRelay>, args: RemoveRelayArgs) -> Result<()> {
        instructions::register_relay::remove_relay_handler(ctx, args)
    }

    /// Verify an aggregated STARK proof and batch-update account states.
    pub fn verify_aggregated(ctx: Context<VerifyAggregated>, args: VerifyAggregatedArgs) -> Result<()> {
        instructions::verify_aggregated::handler(ctx, args)
    }
}
