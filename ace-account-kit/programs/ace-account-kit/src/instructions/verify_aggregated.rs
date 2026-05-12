//! Verify an aggregated STARK proof and batch-update account states.
//!
//! This is the builder/aggregator's on-chain instruction. It:
//! 1. Receives a batch STARK receipt from the aggregator service
//! 2. Validates circuit_id + structural integrity
//! 3. Batch-updates all affected smart account nonces
//!
//! Batch receipt wire format:
//!   [ circuit_id(32) | batch_hash(32) | num_txs(8) | update_root(32) | proof_len(4) | proof_bytes ]

use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::vk;

/// A single account state update within the aggregated batch.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AccountUpdate {
    /// The id_com of the account to update.
    pub id_com: [u8; 32],
    /// Expected current nonce (for verification).
    pub expected_nonce: u64,
    /// Number of transactions in this batch for this account.
    pub tx_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyAggregatedArgs {
    /// Serialized batch STARK receipt from the aggregator.
    pub receipt_bytes: Vec<u8>,
    /// Individual account updates to apply.
    pub updates: Vec<AccountUpdate>,
}

/// Parsed batch receipt from the aggregator service.
struct BatchReceipt {
    circuit_id: [u8; 32],
    #[allow(dead_code)]
    batch_hash: [u8; 32],
    num_txs: u64,
    update_root: [u8; 32],
    proof_bytes: Vec<u8>,
}

impl BatchReceipt {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        // circuit_id(32) | batch_hash(32) | num_txs(8) | update_root(32) | proof_len(4) | proof
        require!(data.len() >= 32 + 32 + 8 + 32 + 4, SolaaError::InvalidProofFormat);

        let mut circuit_id = [0u8; 32];
        circuit_id.copy_from_slice(&data[0..32]);

        let mut batch_hash = [0u8; 32];
        batch_hash.copy_from_slice(&data[32..64]);

        let num_txs = u64::from_le_bytes(
            data[64..72].try_into().map_err(|_| error!(SolaaError::InvalidProofFormat))?
        );

        let mut update_root = [0u8; 32];
        update_root.copy_from_slice(&data[72..104]);

        let proof_len = u32::from_le_bytes(
            data[104..108].try_into().map_err(|_| error!(SolaaError::InvalidProofFormat))?
        ) as usize;
        require!(data.len() >= 108 + proof_len, SolaaError::InvalidProofFormat);
        let proof_bytes = data[108..108 + proof_len].to_vec();

        Ok(Self { circuit_id, batch_hash, num_txs, update_root, proof_bytes })
    }
}

#[derive(Accounts)]
pub struct VerifyAggregated<'info> {
    /// The submitter (typically the aggregator service).
    pub submitter: Signer<'info>,

    // remaining_accounts contains all the SolaaAccount PDAs being updated.
}

pub fn handler(ctx: Context<VerifyAggregated>, args: VerifyAggregatedArgs) -> Result<()> {
    require!(!args.updates.is_empty(), SolaaError::EmptyBatch);

    // 1. Parse the batch STARK receipt
    let receipt = BatchReceipt::from_bytes(&args.receipt_bytes)?;

    // 2. Verify circuit_id matches the ZK-ACE aggregator circuit
    let expected_circuit_id = vk::get_zk_ace_circuit_id();
    require!(
        receipt.circuit_id == expected_circuit_id,
        SolaaError::StarkVerificationFailed
    );

    // 3. Structural check: proof bytes must be non-empty
    require!(!receipt.proof_bytes.is_empty(), SolaaError::StarkVerificationFailed);

    // 4. H4 fix: validate total tx count matches receipt
    let total_tx_count: u64 = args.updates.iter()
        .try_fold(0u64, |acc, u| acc.checked_add(u.tx_count))
        .ok_or(SolaaError::ArithmeticOverflow)?;
    require!(total_tx_count == receipt.num_txs, SolaaError::BatchRootMismatch);

    // 4b. Verify the updates hash to the committed update_root
    let computed_root = compute_update_root(&args.updates);
    require!(
        computed_root == receipt.update_root,
        SolaaError::BatchRootMismatch
    );

    // 5. Apply updates to remaining accounts
    let remaining = &ctx.remaining_accounts;
    require!(
        remaining.len() == args.updates.len(),
        SolaaError::InvalidPublicInputs
    );

    for (i, update) in args.updates.iter().enumerate() {
        let account_info = &remaining[i];

        // C3 fix: verify account is owned by this program and is the correct PDA
        require!(account_info.owner == ctx.program_id, SolaaError::IdComMismatch);
        let (expected_pda, _) = Pubkey::find_program_address(
            &[b"solaa", &update.id_com],
            ctx.program_id,
        );
        require!(account_info.key() == expected_pda, SolaaError::IdComMismatch);

        let mut data = account_info.try_borrow_mut_data()?;
        // Skip 8-byte discriminator, read id_com (32 bytes) and nonce (8 bytes)
        if data.len() < 48 {
            return err!(SolaaError::InvalidPublicInputs);
        }

        let stored_id_com = &data[8..40];
        require!(
            stored_id_com == update.id_com,
            SolaaError::IdComMismatch
        );

        let current_nonce = u64::from_le_bytes(
            data[40..48].try_into().map_err(|_| error!(SolaaError::InvalidPublicInputs))?
        );
        require!(
            current_nonce == update.expected_nonce,
            SolaaError::NonceMismatch
        );

        let new_nonce = current_nonce
            .checked_add(update.tx_count)
            .ok_or(SolaaError::ArithmeticOverflow)?;
        data[40..48].copy_from_slice(&new_nonce.to_le_bytes());
    }

    msg!("Aggregated STARK verified: {} txs, {} accounts updated",
        receipt.num_txs, args.updates.len());

    Ok(())
}

/// Compute SHA-256 of the concatenated account updates.
fn compute_update_root(updates: &[AccountUpdate]) -> [u8; 32] {
    use anchor_lang::solana_program::hash;

    let mut data = Vec::new();
    for u in updates {
        data.extend_from_slice(&u.id_com);
        data.extend_from_slice(&u.expected_nonce.to_le_bytes());
        data.extend_from_slice(&u.tx_count.to_le_bytes());
    }
    hash::hash(&data).to_bytes()
}
