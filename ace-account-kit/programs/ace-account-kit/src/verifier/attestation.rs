//! AR-ACE attestation verification.
//!
//! Attestation message: obj_hash(32) || domain(8, LE) || nonce(8, LE)
//! Signature = Ed25519.Sign(sk_relay, message)
//!
//! On-chain: the caller must prepend an Ed25519Program instruction in the same
//! transaction. This instruction is verified by the Solana runtime before ours
//! executes. We then confirm via the Instructions sysvar that it covers the exact
//! (pubkey, message, signature) triple we expect.

use anchor_lang::prelude::*;
use crate::errors::SolaaError;

/// Ed25519 program ID (native precompile on Solana).
const ED25519_PROGRAM_ID: Pubkey = anchor_lang::solana_program::ed25519_program::ID;

/// Reconstruct the attestation message from components.
pub fn build_attestation_message(
    obj_hash: &[u8; 32],
    domain: u64,
    nonce: u64,
) -> [u8; 48] {
    let mut msg = [0u8; 48];
    msg[0..32].copy_from_slice(obj_hash);
    msg[32..40].copy_from_slice(&domain.to_le_bytes());
    msg[40..48].copy_from_slice(&nonce.to_le_bytes());
    msg
}

/// Parse Ed25519 instruction data and check that it contains our expected
/// (pubkey, message, signature) triple.
///
/// Ed25519 instruction data layout (see solana-program/src/ed25519_instruction.rs):
///   [0]:    num_signatures (u8)
///   [1]:    padding (u8) = 0
///   For each sig i (header at byte 2 + 14*i):
///     [0..2]:  signature_offset (u16 LE) — offset in instruction data
///     [2..4]:  signature_instruction_index (u16 LE)
///     [4..6]:  public_key_offset (u16 LE) — offset in instruction data
///     [6..8]:  public_key_instruction_index (u16 LE)
///     [8..10]: message_data_offset (u16 LE)
///     [10..12]:message_data_size (u16 LE)
///     [12..14]:message_instruction_index (u16 LE)
fn ed25519_ix_contains(
    ix_data: &[u8],
    expected_pubkey: &[u8; 32],
    expected_message: &[u8],
    expected_sig: &[u8; 64],
) -> bool {
    if ix_data.len() < 2 {
        return false;
    }
    let num_sigs = ix_data[0] as usize;
    let header_end = 2 + num_sigs * 14;
    if ix_data.len() < header_end {
        return false;
    }

    for i in 0..num_sigs {
        let base = 2 + i * 14;

        let sig_offset = u16::from_le_bytes([ix_data[base], ix_data[base + 1]]) as usize;
        let sig_ix_idx = u16::from_le_bytes([ix_data[base + 2], ix_data[base + 3]]);
        let pk_offset = u16::from_le_bytes([ix_data[base + 4], ix_data[base + 5]]) as usize;
        let pk_ix_idx = u16::from_le_bytes([ix_data[base + 6], ix_data[base + 7]]);
        let msg_offset = u16::from_le_bytes([ix_data[base + 8], ix_data[base + 9]]) as usize;
        let msg_size = u16::from_le_bytes([ix_data[base + 10], ix_data[base + 11]]) as usize;
        let msg_ix_idx = u16::from_le_bytes([ix_data[base + 12], ix_data[base + 13]]);

        // All offsets must reference the same instruction (0xFFFF = current)
        const CURRENT: u16 = u16::MAX;
        if (sig_ix_idx != 0 && sig_ix_idx != CURRENT)
            || (pk_ix_idx != 0 && pk_ix_idx != CURRENT)
            || (msg_ix_idx != 0 && msg_ix_idx != CURRENT)
        {
            continue;
        }

        // Bounds check
        if ix_data.len() < sig_offset + 64
            || ix_data.len() < pk_offset + 32
            || ix_data.len() < msg_offset + msg_size
        {
            continue;
        }

        if msg_size != expected_message.len() {
            continue;
        }

        let sig_match = &ix_data[sig_offset..sig_offset + 64] == expected_sig.as_ref();
        let pk_match = &ix_data[pk_offset..pk_offset + 32] == expected_pubkey.as_ref();
        let msg_match = &ix_data[msg_offset..msg_offset + msg_size] == expected_message;

        if sig_match && pk_match && msg_match {
            return true;
        }
    }
    false
}

/// Verify an Ed25519 relay attestation.
///
/// Off-chain (not `target_os = "solana"`): uses ed25519-dalek directly.
/// On-chain: requires an Ed25519Program instruction in the same transaction
/// that pre-verifies the signature. We confirm via the Instructions sysvar.
pub fn verify_attestation(
    instructions_sysvar: &AccountInfo,
    signature: &[u8; 64],
    relay_pubkey: &[u8; 32],
    obj_hash: &[u8; 32],
    domain: u64,
    nonce: u64,
) -> Result<bool> {
    let message = build_attestation_message(obj_hash, domain, nonce);

    #[cfg(not(target_os = "solana"))]
    {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};
        let _ = instructions_sysvar; // unused off-chain
        let pubkey = VerifyingKey::from_bytes(relay_pubkey)
            .map_err(|_| error!(SolaaError::InvalidAttestation))?;
        let sig = Signature::from_bytes(signature);
        pubkey
            .verify(&message, &sig)
            .map_err(|_| error!(SolaaError::InvalidAttestation))?;
        return Ok(true);
    }

    #[cfg(target_os = "solana")]
    {
        use anchor_lang::solana_program::sysvar::instructions::{
            load_current_index_checked, load_instruction_at_checked,
        };

        let current_idx = load_current_index_checked(instructions_sysvar)?;

        // Scan all preceding instructions for an Ed25519Program entry that
        // covers our exact (pubkey, message, signature) triple.
        for i in 0..current_idx {
            let ix = load_instruction_at_checked(i as usize, instructions_sysvar)
                .map_err(|_| error!(SolaaError::InvalidAttestation))?;

            if ix.program_id != ED25519_PROGRAM_ID {
                continue;
            }

            if ed25519_ix_contains(&ix.data, relay_pubkey, &message, signature) {
                return Ok(true);
            }
        }

        return Err(error!(SolaaError::InvalidAttestation));
    }
}

/// Check if a relay pubkey is in the authorized set.
pub fn is_relay_authorized(
    relay_pubkey: &[u8; 32],
    authorized_relays: &[[u8; 32]],
) -> bool {
    authorized_relays.iter().any(|r| r == relay_pubkey)
}
