use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::state::{SolaaAccount, PendingRecovery};
use crate::verifier::types::{ProofData, VkType, verify_proof};
use crate::verifier::public_inputs::{ZkAcePublicInputs, NUM_PUBLIC_INPUTS, nonce_to_field};

// ─── Initiate Recovery ───────────────────────────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitiateRecoveryArgs {
    pub seed_id_com: [u8; 32],
    pub new_id_com: [u8; 32],
}

#[derive(Accounts)]
#[instruction(args: InitiateRecoveryArgs)]
pub struct InitiateRecovery<'info> {
    #[account(mut, seeds = [SolaaAccount::SEED_PREFIX, args.seed_id_com.as_ref()], bump = smart_account.bump)]
    pub smart_account: Account<'info, SolaaAccount>,

    /// The guardian must sign to initiate recovery.
    pub guardian: Signer<'info>,
}

pub fn handler_initiate(
    ctx: Context<InitiateRecovery>,
    args: InitiateRecoveryArgs,
) -> Result<()> {
    let account = &mut ctx.accounts.smart_account;

    // Verify guardian
    let guardian = account.guardian.ok_or(SolaaError::NoGuardian)?;
    require!(
        ctx.accounts.guardian.key() == guardian,
        SolaaError::UnauthorizedGuardian
    );

    // No double-recovery
    require!(
        account.pending_recovery.is_none(),
        SolaaError::RecoveryAlreadyPending
    );

    let clock = Clock::get()?;
    account.pending_recovery = Some(PendingRecovery {
        new_id_com: args.new_id_com,
        initiated_at: clock.slot,
    });

    msg!("Recovery initiated by guardian");
    msg!("  new_id_com: {:?}", &args.new_id_com[..8]);
    msg!("  timelock until slot: {}", clock.slot + account.recovery_delay);

    Ok(())
}

// ─── Finalize Recovery ───────────────────────────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FinalizeRecoveryArgs {
    pub seed_id_com: [u8; 32],
}

#[derive(Accounts)]
#[instruction(args: FinalizeRecoveryArgs)]
pub struct FinalizeRecovery<'info> {
    #[account(mut, seeds = [SolaaAccount::SEED_PREFIX, args.seed_id_com.as_ref()], bump = smart_account.bump)]
    pub smart_account: Account<'info, SolaaAccount>,

    /// Anyone can finalize after the timelock — it's permissionless.
    pub caller: Signer<'info>,
}

pub fn handler_finalize(ctx: Context<FinalizeRecovery>, _args: FinalizeRecoveryArgs) -> Result<()> {
    let account = &mut ctx.accounts.smart_account;

    let pending = account.pending_recovery
        .as_ref()
        .ok_or(SolaaError::NoPendingRecovery)?;

    // Check timelock
    let clock = Clock::get()?;
    let elapsed = clock.slot.saturating_sub(pending.initiated_at);
    require!(
        elapsed >= account.recovery_delay,
        SolaaError::TimelockNotElapsed
    );

    // Apply recovery
    let new_id_com = pending.new_id_com;
    account.id_com = new_id_com;
    account.pending_recovery = None;
    account.nonce = account.nonce
        .checked_add(1)
        .ok_or(SolaaError::ArithmeticOverflow)?;

    msg!("Recovery finalized!");
    msg!("  new id_com: {:?}", &new_id_com[..8]);

    Ok(())
}

// ─── Cancel Recovery ─────────────────────────────────────────────────

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CancelRecoveryArgs {
    pub seed_id_com: [u8; 32],
    /// Proof of current ownership (to cancel).
    pub proof: ProofData,
    pub public_inputs_bytes: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: CancelRecoveryArgs)]
pub struct CancelRecovery<'info> {
    #[account(mut, seeds = [SolaaAccount::SEED_PREFIX, args.seed_id_com.as_ref()], bump = smart_account.bump)]
    pub smart_account: Account<'info, SolaaAccount>,

    pub submitter: Signer<'info>,
}

pub fn handler_cancel(
    ctx: Context<CancelRecovery>,
    args: CancelRecoveryArgs,
) -> Result<()> {
    let account = &mut ctx.accounts.smart_account;

    require!(
        account.pending_recovery.is_some(),
        SolaaError::NoPendingRecovery
    );

    // H3 fix: parse public_inputs_bytes with nonce field
    require!(
        args.public_inputs_bytes.len() >= (NUM_PUBLIC_INPUTS + 1) * 32,
        SolaaError::InvalidPublicInputs
    );
    let pub_inputs = ZkAcePublicInputs::from_bytes(&args.public_inputs_bytes[..NUM_PUBLIC_INPUTS * 32])?;
    require!(pub_inputs.id_com == account.id_com, SolaaError::IdComMismatch);

    // Verify nonce to prevent replay
    let nonce_field = &args.public_inputs_bytes[NUM_PUBLIC_INPUTS * 32..(NUM_PUBLIC_INPUTS + 1) * 32];
    let submitted_nonce = u64::from_be_bytes(
        nonce_field[24..32].try_into().map_err(|_| error!(SolaaError::InvalidPublicInputs))?
    );
    require!(submitted_nonce == account.nonce, SolaaError::NonceMismatch);
    require!(pub_inputs.rp_com == nonce_to_field(submitted_nonce), SolaaError::NonceMismatch);

    let field_elements = pub_inputs.as_field_elements();
    let inputs: Vec<[u8; 32]> = field_elements.to_vec();
    let valid = verify_proof(&args.proof, &inputs, VkType::ZkAce)?;
    require!(valid, SolaaError::ProofVerificationFailed);

    account.pending_recovery = None;
    // Bump nonce so this cancel proof cannot be replayed against a future recovery
    // initiated at the same nonce level.
    account.nonce = account.nonce
        .checked_add(1)
        .ok_or(SolaaError::ArithmeticOverflow)?;

    msg!("Recovery cancelled by owner (proof verified)");
    msg!("  nonce bumped to: {}", account.nonce);

    Ok(())
}
