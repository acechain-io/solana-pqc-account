use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash as sha256;
use anchor_lang::solana_program::system_instruction;
use crate::errors::SolaaError;
use crate::state::SolaaAccount;
use crate::verifier::types::{ProofData, VkType, verify_proof};
use crate::verifier::public_inputs::{ZkAcePublicInputs, NUM_PUBLIC_INPUTS, nonce_to_field};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ExecuteArgs {
    /// The original id_com used as the PDA seed (frozen at initialization).
    pub seed_id_com: [u8; 32],
    /// The transaction payload to authorize (variable length).
    pub payload: Vec<u8>,
    /// Proof data (STARK or Attestation).
    pub proof: ProofData,
    /// Public inputs: 5 × 32 (proof fields) + 32 (nonce) = 192 bytes.
    /// Nonce field is the 6th element: u64 big-endian in last 8 bytes of a 32-byte array.
    pub public_inputs_bytes: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: ExecuteArgs)]
pub struct Execute<'info> {
    #[account(
        mut,
        seeds = [SolaaAccount::SEED_PREFIX, args.seed_id_com.as_ref()],
        bump = smart_account.bump,
    )]
    pub smart_account: Account<'info, SolaaAccount>,

    /// Anyone can submit a proof — no signer check needed.
    /// Authorization comes from the ZK proof, not from a signature.
    pub submitter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Execute>, args: ExecuteArgs) -> Result<()> {
    // Copy immutable fields needed for CPI signer seeds before mutable borrow
    let seed_id_com = ctx.accounts.smart_account.seed_id_com;
    let account_bump = ctx.accounts.smart_account.bump;
    let account_key = ctx.accounts.smart_account.key();
    let account = &mut ctx.accounts.smart_account;

    // public_inputs_bytes = 5 × 32 (proof fields) + 32 (nonce field) = 192 bytes
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

    // 3a. M1 fix: rp_com must equal nonce_to_field(nonce) — links proof to nonce
    require!(
        pub_inputs.rp_com == nonce_to_field(submitted_nonce),
        SolaaError::NonceMismatch
    );

    // 4. Verify payload hash: SHA-256(payload) must match pub_inputs.tx_hash
    let payload_hash = sha256(&args.payload);
    require!(
        payload_hash.to_bytes() == pub_inputs.tx_hash,
        SolaaError::PayloadHashMismatch
    );

    // 5. Verify id_com matches on-chain state
    require!(pub_inputs.id_com == account.id_com, SolaaError::IdComMismatch);

    // 6. Verify domain matches
    let mut expected_domain = [0u8; 32];
    expected_domain[24..32].copy_from_slice(&account.domain.to_be_bytes());
    require!(pub_inputs.domain == expected_domain, SolaaError::DomainMismatch);

    // 7. Verify the STARK proof
    let field_elements = pub_inputs.as_field_elements();
    let valid = verify_proof(&args.proof, &field_elements.to_vec(), VkType::ZkAce)?;
    require!(valid, SolaaError::ProofVerificationFailed);

    // H5 fix: CPI dispatch for SOL transfer: [destination(32) | amount_le(8)] = 40 bytes
    if args.payload.len() == 40 {
        let dest_bytes: [u8; 32] = args.payload[..32].try_into()
            .map_err(|_| error!(SolaaError::InvalidPublicInputs))?;
        let dest = Pubkey::new_from_array(dest_bytes);
        let amount = u64::from_le_bytes(
            args.payload[32..40].try_into()
                .map_err(|_| error!(SolaaError::InvalidPublicInputs))?
        );
        if amount > 0 {
            // Drop mutable borrow before calling to_account_info
            drop(account);
            let ix = system_instruction::transfer(
                &account_key,
                &dest,
                amount,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[ctx.accounts.smart_account.to_account_info()],
                &[&[
                    SolaaAccount::SEED_PREFIX,
                    seed_id_com.as_ref(),
                    &[account_bump],
                ]],
            )?;
            // Re-borrow after CPI
            let account = &mut ctx.accounts.smart_account;
            account.nonce = account.nonce
                .checked_add(1)
                .ok_or(SolaaError::ArithmeticOverflow)?;
            msg!("ZK-ACE execute: proof verified, nonce={}", account.nonce);
            msg!("  payload_len={}", args.payload.len());
            return Ok(());
        }
    }

    // 8. Bump nonce
    let account = &mut ctx.accounts.smart_account;
    account.nonce = account.nonce
        .checked_add(1)
        .ok_or(SolaaError::ArithmeticOverflow)?;

    msg!("ZK-ACE execute: proof verified, nonce={}", account.nonce);
    msg!("  payload_len={}", args.payload.len());

    Ok(())
}
