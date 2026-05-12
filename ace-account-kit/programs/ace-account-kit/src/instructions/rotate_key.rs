use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::state::SolaaAccount;
use crate::verifier::types::{ProofData, VkType, verify_proof};
use crate::verifier::public_inputs::{ZkAcePublicInputs, NUM_PUBLIC_INPUTS, nonce_to_field};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RotateKeyArgs {
    /// The original id_com used as the PDA seed (frozen at initialization).
    pub seed_id_com: [u8; 32],
    /// The new identity commitment (e.g. derived from ML-DSA-44 key).
    pub new_id_com: [u8; 32],
    /// Proof authorizing the rotation (proves current REV ownership).
    pub proof: ProofData,
    /// Public inputs: 5 × 32 (proof fields) + 32 (nonce) = 192 bytes.
    pub public_inputs_bytes: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: RotateKeyArgs)]
pub struct RotateKey<'info> {
    #[account(
        mut,
        seeds = [SolaaAccount::SEED_PREFIX, args.seed_id_com.as_ref()],
        bump = smart_account.bump,
    )]
    pub smart_account: Account<'info, SolaaAccount>,

    pub submitter: Signer<'info>,
}

pub fn handler(ctx: Context<RotateKey>, args: RotateKeyArgs) -> Result<()> {
    let account = &mut ctx.accounts.smart_account;

    require!(
        args.public_inputs_bytes.len() == (NUM_PUBLIC_INPUTS + 1) * 32,
        SolaaError::InvalidPublicInputs
    );

    // 1. Parse the first 5 × 32 bytes as proof public inputs
    let pub_inputs = ZkAcePublicInputs::from_bytes(&args.public_inputs_bytes[..NUM_PUBLIC_INPUTS * 32])?;

    // 2. Parse the 6th element (nonce): u64 big-endian from last 8 bytes
    let nonce_field = &args.public_inputs_bytes[NUM_PUBLIC_INPUTS * 32..(NUM_PUBLIC_INPUTS + 1) * 32];
    let submitted_nonce = u64::from_be_bytes(
        nonce_field[24..32].try_into().map_err(|_| error!(SolaaError::InvalidPublicInputs))?
    );

    // 3. Verify nonce matches on-chain state (replay prevention)
    require!(submitted_nonce == account.nonce, SolaaError::NonceMismatch);

    // M1 fix: rp_com must equal nonce_to_field(nonce)
    require!(pub_inputs.rp_com == nonce_to_field(submitted_nonce), SolaaError::NonceMismatch);

    // 4. The proof must be against the CURRENT id_com (proves current ownership)
    require!(pub_inputs.id_com == account.id_com, SolaaError::IdComMismatch);

    // 5. Verify domain
    let mut expected_domain = [0u8; 32];
    expected_domain[24..32].copy_from_slice(&account.domain.to_be_bytes());
    require!(pub_inputs.domain == expected_domain, SolaaError::DomainMismatch);

    // 6. Verify the STARK proof
    let field_elements = pub_inputs.as_field_elements();
    let valid = verify_proof(&args.proof, &field_elements.to_vec(), VkType::ZkAce)?;
    require!(valid, SolaaError::ProofVerificationFailed);

    // 7. Rotate: update id_com
    let old_id_com = account.id_com;
    account.id_com = args.new_id_com;

    // 8. Bump nonce
    account.nonce = account.nonce
        .checked_add(1)
        .ok_or(SolaaError::ArithmeticOverflow)?;

    // 9. Clear any pending recovery (rotation invalidates it)
    account.pending_recovery = None;

    msg!("Key rotated!");
    msg!("  old id_com: {:?}", &old_id_com[..8]);
    msg!("  new id_com: {:?}", &args.new_id_com[..8]);
    msg!("  nonce: {}", account.nonce);

    Ok(())
}
