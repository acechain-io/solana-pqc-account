#[allow(deprecated)]
use solana_client::rpc_client::RpcClient;
#[allow(deprecated)]
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

use crate::error::{SolaaClientError, Result};
use crate::proof_generator::{SolaaProofResult, AttestationData};

pub struct SolaaClient {
    pub rpc: RpcClient,
    pub program_id: Pubkey,
    pub payer: Keypair,
}

const PDA_SEED: &[u8] = b"solaa";

/// Circuit ID matching on-chain vk::get_zk_ace_circuit_id()
const ZK_ACE_CIRCUIT_ID: [u8; 32] = [
    0xcf, 0x15, 0xa0, 0xe5, 0xb4, 0xb3, 0xa0, 0xb3,
    0x27, 0xf7, 0xfc, 0xc6, 0x6b, 0x21, 0x39, 0x63,
    0x88, 0x75, 0xae, 0x15, 0x77, 0xa9, 0xd9, 0x73,
    0x5e, 0xe6, 0x53, 0x58, 0xb1, 0x0b, 0x7b, 0xe8,
];

impl SolaaClient {
    pub fn new(rpc_url: &str, program_id: Pubkey, payer: Keypair) -> Self {
        Self { rpc: RpcClient::new(rpc_url.to_string()), program_id, payer }
    }

    pub fn get_pda(program_id: &Pubkey, id_com: &[u8; 32]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[PDA_SEED, id_com.as_ref()], program_id)
    }

    pub fn build_initialize_tx(
        &self,
        id_com: &[u8; 32],
        domain: u64,
        guardian: Option<Pubkey>,
        recovery_delay: Option<u64>,
    ) -> Result<Transaction> {
        let (pda, _) = Self::get_pda(&self.program_id, id_com);
        let mut data = anchor_discriminator("initialize").to_vec();
        // InitializeArgs borsh: id_com(32) | domain(u64 le) | Option<Pubkey> | Option<u64>
        data.extend_from_slice(id_com);
        data.extend_from_slice(&domain.to_le_bytes());
        match guardian {
            None => data.push(0),
            Some(g) => { data.push(1); data.extend_from_slice(g.as_ref()); }
        }
        match recovery_delay {
            None => data.push(0),
            Some(d) => { data.push(1); data.extend_from_slice(&d.to_le_bytes()); }
        }
        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        self.make_tx(&[Instruction::new_with_bytes(self.program_id, &data, accounts)], &[])
    }

    pub fn build_execute_tx(
        &self,
        seed_id_com: &[u8; 32],
        proof_result: &SolaaProofResult,
        payload: &[u8],
    ) -> Result<Transaction> {
        let (pda, _) = Self::get_pda(&self.program_id, seed_id_com);
        let receipt_bytes = build_stark_receipt(proof_result);
        let public_inputs_bytes = build_public_inputs_bytes(proof_result);

        let mut data = anchor_discriminator("execute").to_vec();
        // ExecuteArgs borsh: seed_id_com(32) | payload(vec) | proof(enum) | public_inputs_bytes(vec)
        data.extend_from_slice(seed_id_com);
        write_vec_u8(&mut data, payload);
        write_proof_stark(&mut data, &receipt_bytes);
        write_vec_u8(&mut data, &public_inputs_bytes);

        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        self.make_tx(&[Instruction::new_with_bytes(self.program_id, &data, accounts)], &[])
    }

    pub fn build_rotate_key_tx(
        &self,
        seed_id_com: &[u8; 32],
        new_id_com: &[u8; 32],
        proof_result: &SolaaProofResult,
    ) -> Result<Transaction> {
        let (pda, _) = Self::get_pda(&self.program_id, seed_id_com);
        let receipt_bytes = build_stark_receipt(proof_result);
        let public_inputs_bytes = build_public_inputs_bytes(proof_result);

        let mut data = anchor_discriminator("rotate_key").to_vec();
        // RotateKeyArgs borsh: seed_id_com(32) | new_id_com(32) | proof(enum) | pub_inputs(vec)
        data.extend_from_slice(seed_id_com);
        data.extend_from_slice(new_id_com);
        write_proof_stark(&mut data, &receipt_bytes);
        write_vec_u8(&mut data, &public_inputs_bytes);

        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
        ];
        self.make_tx(&[Instruction::new_with_bytes(self.program_id, &data, accounts)], &[])
    }

    pub fn build_initiate_recovery_tx(
        &self,
        seed_id_com: &[u8; 32],
        new_id_com: &[u8; 32],
        guardian: &Keypair,
    ) -> Result<Transaction> {
        let (pda, _) = Self::get_pda(&self.program_id, seed_id_com);
        let mut data = anchor_discriminator("initiate_recovery").to_vec();
        // InitiateRecoveryArgs borsh: seed_id_com(32) | new_id_com(32)
        data.extend_from_slice(seed_id_com);
        data.extend_from_slice(new_id_com);
        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(guardian.pubkey(), true),
        ];
        self.make_tx(&[Instruction::new_with_bytes(self.program_id, &data, accounts)], &[guardian])
    }

    pub fn build_finalize_recovery_tx(&self, seed_id_com: &[u8; 32]) -> Result<Transaction> {
        let (pda, _) = Self::get_pda(&self.program_id, seed_id_com);
        let mut data = anchor_discriminator("finalize_recovery").to_vec();
        // FinalizeRecoveryArgs borsh: seed_id_com(32)
        data.extend_from_slice(seed_id_com);
        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
        ];
        self.make_tx(&[Instruction::new_with_bytes(self.program_id, &data, accounts)], &[])
    }

    pub fn build_execute_attested_tx(
        &self,
        seed_id_com: &[u8; 32],
        attestation: &AttestationData,
        payload: &[u8],
        relay_registry: &Pubkey,
    ) -> Result<Transaction> {
        use solana_sdk::sysvar;

        let (pda, _) = Self::get_pda(&self.program_id, seed_id_com);

        // Ed25519 attestation message: obj_hash(32) || domain.le(8) || nonce.le(8)
        let mut attest_msg = [0u8; 48];
        attest_msg[..32].copy_from_slice(&attestation.obj_hash);
        attest_msg[32..40].copy_from_slice(&attestation.domain.to_le_bytes());
        attest_msg[40..48].copy_from_slice(&attestation.nonce.to_le_bytes());

        let ed25519_ix = build_ed25519_ix(&attestation.relay_pubkey, &attestation.signature, &attest_msg);

        let mut data = anchor_discriminator("execute_attested").to_vec();
        // ExecuteAttestedArgs borsh: seed_id_com(32)|payload(vec)|sig(64)|relay_pk(32)|obj_hash(32)|domain(u64)|attest_nonce(u64)
        data.extend_from_slice(seed_id_com);
        write_vec_u8(&mut data, payload);
        data.extend_from_slice(&attestation.signature);
        data.extend_from_slice(&attestation.relay_pubkey);
        data.extend_from_slice(&attestation.obj_hash);
        data.extend_from_slice(&attestation.domain.to_le_bytes());
        data.extend_from_slice(&attestation.nonce.to_le_bytes());

        let accounts = vec![
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(*relay_registry, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
        ];
        let main_ix = Instruction::new_with_bytes(self.program_id, &data, accounts);
        self.make_tx(&[ed25519_ix, main_ix], &[])
    }

    fn make_tx(&self, instructions: &[Instruction], extra_signers: &[&Keypair]) -> Result<Transaction> {
        let blockhash = self.rpc.get_latest_blockhash()
            .map_err(|e| SolaaClientError::Rpc(e.to_string()))?;
        let mut signers: Vec<&Keypair> = vec![&self.payer];
        signers.extend_from_slice(extra_signers);
        Ok(Transaction::new_signed_with_payer(
            instructions,
            Some(&self.payer.pubkey()),
            &signers,
            blockhash,
        ))
    }
}

// ─── helpers ──────────────────────────────────────────────────────────────

fn anchor_discriminator(name: &str) -> [u8; 8] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(format!("global:{}", name).as_bytes());
    let hash = h.finalize();
    let mut d = [0u8; 8];
    d.copy_from_slice(&hash[..8]);
    d
}

fn write_vec_u8(buf: &mut Vec<u8>, v: &[u8]) {
    buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
    buf.extend_from_slice(v);
}

fn write_proof_stark(buf: &mut Vec<u8>, receipt_bytes: &[u8]) {
    buf.push(0u8); // ProofData::Stark variant index
    write_vec_u8(buf, receipt_bytes);
}

fn domain_to_field(domain: u64) -> [u8; 32] {
    let mut f = [0u8; 32];
    f[24..32].copy_from_slice(&domain.to_be_bytes());
    f
}

fn nonce_to_field(nonce: u64) -> [u8; 32] {
    let mut f = [0u8; 32];
    f[24..32].copy_from_slice(&nonce.to_be_bytes());
    f
}

/// Build ZkAceStarkReceipt wire format:
/// circuit_id(32) | num_inputs(1) | inputs(N×32) | proof_len(4) | proof_bytes
fn build_stark_receipt(proof_result: &SolaaProofResult) -> Vec<u8> {
    let inputs: [&[u8; 32]; 5] = [
        &proof_result.public_inputs.id_com,
        &proof_result.public_inputs.tx_hash,
        &domain_to_field(proof_result.public_inputs.domain),
        &proof_result.public_inputs.target,
        &proof_result.public_inputs.rp_com,
    ];
    let mut out = Vec::new();
    out.extend_from_slice(&ZK_ACE_CIRCUIT_ID);
    out.push(5u8);
    for inp in inputs { out.extend_from_slice(inp); }
    out.extend_from_slice(&(proof_result.proof_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&proof_result.proof_bytes);
    out
}

/// public_inputs_bytes: 5 field elements + nonce field = 192 bytes
fn build_public_inputs_bytes(proof_result: &SolaaProofResult) -> Vec<u8> {
    let mut out = Vec::with_capacity(192);
    out.extend_from_slice(&proof_result.public_inputs.id_com);
    out.extend_from_slice(&proof_result.public_inputs.tx_hash);
    out.extend_from_slice(&domain_to_field(proof_result.public_inputs.domain));
    out.extend_from_slice(&proof_result.public_inputs.target);
    out.extend_from_slice(&proof_result.public_inputs.rp_com);
    out.extend_from_slice(&nonce_to_field(proof_result.nonce));
    out
}

/// Build an Ed25519Program instruction for relay signature pre-verification.
/// Layout: num_sigs(1)|pad(1)|[sig_off(2)|sig_ix(2)|pk_off(2)|pk_ix(2)|msg_off(2)|msg_sz(2)|msg_ix(2)]|sig(64)|pk(32)|msg
fn build_ed25519_ix(pubkey: &[u8; 32], signature: &[u8; 64], message: &[u8]) -> Instruction {
    let header_size: u16 = 2 + 14; // 2 prefix + 14 per sig
    let sig_offset = header_size;
    let pk_offset = sig_offset + 64;
    let msg_offset = pk_offset + 32;
    let msg_size = message.len() as u16;
    const CURRENT_IX: u16 = u16::MAX;

    let mut data = Vec::with_capacity((header_size as usize) + 64 + 32 + message.len());
    data.push(1u8); data.push(0u8); // num_sigs=1, pad
    data.extend_from_slice(&sig_offset.to_le_bytes());
    data.extend_from_slice(&CURRENT_IX.to_le_bytes());
    data.extend_from_slice(&pk_offset.to_le_bytes());
    data.extend_from_slice(&CURRENT_IX.to_le_bytes());
    data.extend_from_slice(&msg_offset.to_le_bytes());
    data.extend_from_slice(&msg_size.to_le_bytes());
    data.extend_from_slice(&CURRENT_IX.to_le_bytes());
    data.extend_from_slice(signature);
    data.extend_from_slice(pubkey);
    data.extend_from_slice(message);

    Instruction {
        program_id: anchor_lang::solana_program::ed25519_program::ID,
        accounts: vec![],
        data,
    }
}
