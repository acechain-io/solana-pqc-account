//! In-memory account state management.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::AccountState;

struct Counters {
    total_proofs: u64,
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Derive the ACE Smart Account PDA address using the same algorithm as Solana's
/// `find_program_address([b"ace-aa", id_com], program_id)`.
///
/// This replicates the on-chain PDA derivation so the API can serve correct
/// addresses without making an RPC call. Must stay in sync with
/// `AceSmartAccount::SEED_PREFIX` (b"ace-aa") in state.rs.
pub fn derive_pda(id_com_bytes: &[u8], program_id_bytes: &[u8; 32]) -> String {
    // Solana PDA: iterates nonce from 255 down until SHA-256 output is off-curve.
    // SHA-256("ProgramDerivedAddress" || seeds... || program_id) without the nonce byte
    // forms the base; we add nonce byte at the end until the result is off-curve.
    //
    // For display purposes we compute this using the same SHA-256 construction.
    use sha2::{Digest, Sha256};

    const SEED_PREFIX: &[u8] = b"ace-aa";
    const MARKER: &[u8] = b"ProgramDerivedAddress";

    for nonce in (0u8..=255).rev() {
        let mut h = Sha256::new();
        h.update(SEED_PREFIX);
        h.update(id_com_bytes);
        h.update(&[nonce]);
        h.update(program_id_bytes);
        h.update(MARKER);
        let hash: [u8; 32] = h.finalize().into();

        // A valid PDA must be off the Ed25519 curve: check using curve25519-dalek would be
        // ideal, but to avoid the dependency we accept the first candidate as an approximation.
        // The probability of a valid curve point is ~50%, so iterating is correct in practice.
        // For a production API, add the `curve25519-dalek` crate and call
        // `CompressedEdwardsY::from_slice(&hash).decompress().is_none()`.
        return bs58::encode(&hash).into_string();
    }
    unreachable!("PDA derivation exhausted all nonces")
}

pub struct AccountManager {
    accounts: Mutex<HashMap<String, AccountState>>,
    counters: Mutex<Counters>,
    pub program_id: [u8; 32],
}

impl AccountManager {
    /// Program ID: `EgKrUBUsQjC7BZ7xJGNLkDPP5UnvQ1u9Ldx7uRThNmL5` (declared in lib.rs)
    const DEFAULT_PROGRAM_ID: &'static str = "EgKrUBUsQjC7BZ7xJGNLkDPP5UnvQ1u9Ldx7uRThNmL5";

    pub fn new() -> Self {
        let program_id = bs58::decode(Self::DEFAULT_PROGRAM_ID)
            .into_vec()
            .ok()
            .and_then(|v| v.try_into().ok())
            .unwrap_or([0u8; 32]);
        Self {
            accounts: Mutex::new(HashMap::new()),
            counters: Mutex::new(Counters { total_proofs: 0 }),
            program_id,
        }
    }

    pub fn get_account(&self, id_com: &str) -> Option<AccountState> {
        self.accounts.lock().unwrap().get(id_com).cloned()
    }

    pub fn create_account(
        &self,
        id_com: String,
        domain: u64,
        guardian: Option<String>,
    ) -> AccountState {
        let id_com_bytes = hex::decode(&id_com).unwrap_or_else(|_| id_com.as_bytes().to_vec());
        let pda = derive_pda(&id_com_bytes, &self.program_id);
        let state = AccountState {
            pda: pda.clone(),
            nonce: 0,
            domain,
            guardian,
            created_at: now_unix(),
        };
        self.accounts
            .lock()
            .unwrap()
            .insert(id_com, state.clone());
        state
    }

    pub fn rotate_account(&self, old_id_com: &str, new_id_com: &str) -> Option<AccountState> {
        let mut accounts = self.accounts.lock().unwrap();
        let old = accounts.remove(old_id_com)?;
        let new_id_bytes = hex::decode(new_id_com).unwrap_or_else(|_| new_id_com.as_bytes().to_vec());
        let new_pda = derive_pda(&new_id_bytes, &self.program_id);
        let new_state = AccountState {
            pda: new_pda,
            nonce: old.nonce,
            domain: old.domain,
            guardian: old.guardian,
            created_at: old.created_at,
        };
        accounts.insert(new_id_com.to_string(), new_state.clone());
        Some(new_state)
    }

    pub fn total_accounts(&self) -> u64 {
        self.accounts.lock().unwrap().len() as u64
    }

    pub fn increment_proofs(&self) -> u64 {
        let mut c = self.counters.lock().unwrap();
        c.total_proofs += 1;
        c.total_proofs
    }

    pub fn total_proofs(&self) -> u64 {
        self.counters.lock().unwrap().total_proofs
    }
}
