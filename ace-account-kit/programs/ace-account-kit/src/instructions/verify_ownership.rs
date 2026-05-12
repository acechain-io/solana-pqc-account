use anchor_lang::prelude::*;
use crate::errors::SolaaError;
use crate::state::OwnershipRecord;
use crate::verifier::types::{ProofData, VkType, verify_proof};
use crate::verifier::public_inputs::ZkOwnershipPublicInputs;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyOwnershipArgs {
    /// The foreign chain address being claimed.
    pub foreign_address: [u8; 32],
    /// Foreign chain identifier (1 = Ethereum, 0 = Bitcoin, etc.).
    pub foreign_chain: u64,
    /// Proof of cross-chain ownership (STARK).
    pub proof: ProofData,
    /// Public inputs (4 × 32 = 128 bytes).
    pub public_inputs_bytes: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: VerifyOwnershipArgs)]
pub struct VerifyOwnership<'info> {
    #[account(
        init,
        payer = payer,
        space = OwnershipRecord::SIZE,
        seeds = [
            OwnershipRecord::SEED_PREFIX,
            args.foreign_address.as_ref(),
            &args.foreign_chain.to_le_bytes(),
        ],
        bump,
    )]
    pub ownership_record: Account<'info, OwnershipRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VerifyOwnership>, args: VerifyOwnershipArgs) -> Result<()> {
    // 1. Parse public inputs
    let pub_inputs = ZkOwnershipPublicInputs::from_bytes(&args.public_inputs_bytes)?;

    // 2. Verify the STARK proof
    let field_elements = pub_inputs.as_field_elements();
    let inputs: Vec<[u8; 32]> = field_elements.to_vec();

    let valid = verify_proof(&args.proof, &inputs, VkType::ZkOwnership)?;
    require!(valid, SolaaError::ProofVerificationFailed);

    // 3. Validate args match the proof's public inputs — prevents an attacker from
    //    submitting a valid proof for address A but recording address B on-chain.
    require!(
        pub_inputs.foreign_address == args.foreign_address,
        SolaaError::ForeignAddressMismatch
    );
    // pub_inputs.foreign_chain is the field element (BE-padded u64).
    let mut expected_chain_field = [0u8; 32];
    expected_chain_field[24..32].copy_from_slice(&args.foreign_chain.to_be_bytes());
    require!(
        pub_inputs.foreign_chain == expected_chain_field,
        SolaaError::ForeignAddressMismatch
    );

    // 4. Store the verified ownership record
    let record = &mut ctx.accounts.ownership_record;
    record.id_com = pub_inputs.id_com;
    record.foreign_address = args.foreign_address;
    record.foreign_chain = args.foreign_chain;
    record.verified_at = Clock::get()?.slot;
    record.bump = ctx.bumps.ownership_record;

    msg!("ZK-Ownership verified!");
    msg!("  id_com: {:?}", &record.id_com[..8]);
    msg!("  foreign_chain: {}", record.foreign_chain);

    Ok(())
}
