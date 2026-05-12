use anchor_lang::prelude::*;

/// SolAA Smart Account — PDA-based account with ZK-ACE authorization.
///
/// PDA seed: `[b"solaa", id_com[0..32]]`
///
/// The address is derived from `program_id` + `id_com`, so rotating
/// the underlying key material (via `rotate_key`) does NOT change
/// the PDA address.  All assets stay put.
#[account]
#[derive(Debug)]
pub struct SolaaAccount {
    /// Identity commitment: Poseidon(REV, salt, domain).
    /// This is the *authorization root* — anyone who can produce
    /// a valid ZK proof against this commitment can operate the account.
    pub id_com: [u8; 32],

    /// The id_com used at initialization time — frozen as the PDA seed.
    /// PDA seed: `[b"solaa", seed_id_com]`. Never changes after init.
    pub seed_id_com: [u8; 32],

    /// Monotonically increasing nonce for replay prevention.
    /// Each successful `execute` or `rotate_key` bumps this by 1.
    pub nonce: u64,

    /// Chain domain tag (prevents cross-domain proof reuse).
    /// Solana mainnet = 1, devnet = 2, etc.
    pub domain: u64,

    /// Optional guardian pubkey for social recovery.
    pub guardian: Option<Pubkey>,

    /// Recovery timelock in slots (~400 ms each).
    /// Default: 1_209_600 slots ≈ 7 days.
    pub recovery_delay: u64,

    /// Pending recovery request (if any).
    pub pending_recovery: Option<PendingRecovery>,

    /// Timestamp when this account was created.
    pub created_at: i64,

    /// PDA bump seed.
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PendingRecovery {
    /// The new id_com proposed by the guardian.
    pub new_id_com: [u8; 32],
    /// The slot at which recovery was initiated.
    pub initiated_at: u64,
}

impl SolaaAccount {
    /// Fixed size for PDA allocation.
    /// 8 (discriminator) + 32 (id_com) + 32 (seed_id_com) + 8 + 8 + 33 + 8 + (1 + 32 + 8) + 8 + 1 = 179
    /// Round up with padding.
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + (1 + 32) + 8 + (1 + 32 + 8) + 8 + 1 + 64;

    pub const SEED_PREFIX: &'static [u8] = b"solaa";
}

/// On-chain record that a ZK-Ownership proof has been verified.
#[account]
#[derive(Debug)]
pub struct OwnershipRecord {
    /// The Solana id_com that owns the foreign address.
    pub id_com: [u8; 32],

    /// The foreign chain address (raw bytes, up to 32).
    pub foreign_address: [u8; 32],

    /// Foreign chain identifier (e.g. 1 = Ethereum, 0 = Bitcoin).
    pub foreign_chain: u64,

    /// Slot when the proof was verified.
    pub verified_at: u64,

    /// PDA bump.
    pub bump: u8,
}

impl OwnershipRecord {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 32;
    pub const SEED_PREFIX: &'static [u8] = b"solaa-own";
}

/// AR-ACE Relay Registry — stores authorized relay node public keys.
///
/// PDA seed: `[b"relay-registry"]`
///
/// The authority can add/remove relay pubkeys. Relay nodes must be
/// registered here before their attestations are accepted.
#[account]
#[derive(Debug)]
pub struct RelayRegistry {
    /// Admin authority who can manage relay registrations.
    pub authority: Pubkey,

    /// Authorized relay Ed25519 public keys.
    pub relays: Vec<[u8; 32]>,

    /// PDA bump.
    pub bump: u8,
}

impl RelayRegistry {
    /// Max relays to keep account size reasonable.
    pub const MAX_RELAYS: usize = 64;

    /// Account size: 8 + 32 + 4 + (32 * MAX_RELAYS) + 1 + padding
    pub const SIZE: usize = 8 + 32 + 4 + (32 * Self::MAX_RELAYS) + 1 + 64;

    pub const SEED_PREFIX: &'static [u8] = b"relay-registry";
}
