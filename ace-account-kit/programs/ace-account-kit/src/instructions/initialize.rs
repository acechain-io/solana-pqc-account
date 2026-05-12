use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::state::SolaaAccount;

/// Default recovery delay: ~7 days at 400 ms/slot.
const DEFAULT_RECOVERY_DELAY: u64 = 1_512_000; // 7 * 24 * 3600 * 1000 / 400

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeArgs {
    /// Identity commitment: Poseidon(REV, salt, domain).
    pub id_com: [u8; 32],
    /// Chain domain tag.
    pub domain: u64,
    /// Optional guardian pubkey for social recovery.
    pub guardian: Option<Pubkey>,
    /// Optional recovery delay in slots (default ~7 days).
    pub recovery_delay: Option<u64>,
}

#[derive(Accounts)]
#[instruction(args: InitializeArgs)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = SolaaAccount::SIZE,
        seeds = [SolaaAccount::SEED_PREFIX, args.id_com.as_ref()],
        bump,
    )]
    pub smart_account: Account<'info, SolaaAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
    let account = &mut ctx.accounts.smart_account;

    account.id_com = args.id_com;
    account.seed_id_com = args.id_com;
    account.nonce = 0;
    account.domain = args.domain;
    account.guardian = args.guardian;
    account.recovery_delay = args.recovery_delay.unwrap_or(DEFAULT_RECOVERY_DELAY);
    require!(account.recovery_delay > 0, SolaaError::TimelockNotElapsed);
    account.pending_recovery = None;
    account.created_at = Clock::get()?.unix_timestamp;
    account.bump = ctx.bumps.smart_account;

    msg!("SolAA Smart Account initialized");
    msg!("  id_com: {:?}", &account.id_com[..8]);
    msg!("  domain: {}", account.domain);
    msg!("  guardian: {:?}", account.guardian);

    Ok(())
}
