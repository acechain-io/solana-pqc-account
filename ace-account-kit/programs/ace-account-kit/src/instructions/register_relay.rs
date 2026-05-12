//! AR-ACE relay registry management.
//!
//! Two separate instructions:
//!   - `initialize_relay_registry`: one-time setup, sets authority (uses `init`)
//!   - `register_relay`: add a relay pubkey (requires stored authority)
//!
//! Splitting init from add removes the race condition where any caller could
//! become authority by calling first with `init_if_needed`.

use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::state::RelayRegistry;

// ---------------------------------------------------------------------------
// initialize_relay_registry
// ---------------------------------------------------------------------------

#[derive(Accounts)]
pub struct InitializeRelayRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = RelayRegistry::SIZE,
        seeds = [RelayRegistry::SEED_PREFIX],
        bump,
    )]
    pub relay_registry: Account<'info, RelayRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_relay_registry_handler(
    ctx: Context<InitializeRelayRegistry>,
) -> Result<()> {
    let registry = &mut ctx.accounts.relay_registry;
    registry.authority = ctx.accounts.authority.key();
    registry.bump = ctx.bumps.relay_registry;
    msg!("Relay registry initialized, authority: {}", registry.authority);
    Ok(())
}

// ---------------------------------------------------------------------------
// register_relay
// ---------------------------------------------------------------------------

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterRelayArgs {
    /// Ed25519 public key of the relay node to register.
    pub relay_pubkey: [u8; 32],
}

#[derive(Accounts)]
pub struct RegisterRelay<'info> {
    #[account(
        mut,
        seeds = [RelayRegistry::SEED_PREFIX],
        bump = relay_registry.bump,
        has_one = authority @ SolaaError::UnauthorizedRelay,
    )]
    pub relay_registry: Account<'info, RelayRegistry>,

    pub authority: Signer<'info>,
}

pub fn register_relay_handler(
    ctx: Context<RegisterRelay>,
    args: RegisterRelayArgs,
) -> Result<()> {
    let registry = &mut ctx.accounts.relay_registry;

    require!(
        registry.relays.len() < RelayRegistry::MAX_RELAYS,
        SolaaError::RelayRegistryFull
    );

    if !registry.relays.contains(&args.relay_pubkey) {
        registry.relays.push(args.relay_pubkey);
    }

    msg!("Relay registered: {:?}", &args.relay_pubkey[..8]);
    msg!("  total relays: {}", registry.relays.len());
    Ok(())
}

// ---------------------------------------------------------------------------
// remove_relay
// ---------------------------------------------------------------------------

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemoveRelayArgs {
    pub relay_pubkey: [u8; 32],
}

#[derive(Accounts)]
pub struct RemoveRelay<'info> {
    #[account(
        mut,
        seeds = [RelayRegistry::SEED_PREFIX],
        bump = relay_registry.bump,
        has_one = authority @ SolaaError::UnauthorizedRelay,
    )]
    pub relay_registry: Account<'info, RelayRegistry>,

    pub authority: Signer<'info>,
}

pub fn remove_relay_handler(ctx: Context<RemoveRelay>, args: RemoveRelayArgs) -> Result<()> {
    let registry = &mut ctx.accounts.relay_registry;
    registry.relays.retain(|r| r != &args.relay_pubkey);
    msg!("Relay removed: {:?}", &args.relay_pubkey[..8]);
    Ok(())
}
