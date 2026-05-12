//! Extract and serialize the Groth16 verifying key for embedding in the on-chain program.

use ark_bn254::Bn254;
use ark_groth16::VerifyingKey;

use crate::convert::{g1_to_solana, g2_to_solana};

/// Solana-format verifying key, ready to embed in vk.rs.
pub struct SolanaVk {
    pub alpha_g1: [u8; 64],
    pub beta_g2: [u8; 128],
    pub gamma_g2: [u8; 128],
    pub delta_g2: [u8; 128],
    /// Length = num_public_inputs + 1. For ZK-ACE: 6 points.
    pub ic: Vec<[u8; 64]>,
}

/// Extract the Groth16 verifying key from dev.zk-ace and convert to Solana format.
pub fn extract_solana_vk(mode: zk_ace::ReplayMode) -> SolanaVk {
    use zk_ace::groth16::prover::get_keys;

    let (_pk, vk) = get_keys(mode);
    vk_to_solana(vk)
}

fn vk_to_solana(vk: &VerifyingKey<Bn254>) -> SolanaVk {
    SolanaVk {
        alpha_g1: g1_to_solana(&vk.alpha_g1),
        beta_g2: g2_to_solana(&vk.beta_g2),
        gamma_g2: g2_to_solana(&vk.gamma_g2),
        delta_g2: g2_to_solana(&vk.delta_g2),
        ic: vk.gamma_abc_g1.iter().map(g1_to_solana).collect(),
    }
}

/// Render the SolanaVk as a Rust source snippet for embedding in vk.rs.
pub fn render_vk_rust(vk: &SolanaVk, fn_name: &str) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "pub fn {}() -> crate::verifier::groth16::VerifyingKey {{\n",
        fn_name
    ));
    s.push_str("    crate::verifier::groth16::VerifyingKey {\n");

    s.push_str(&format!(
        "        alpha_g1: {},\n",
        bytes64_literal(&vk.alpha_g1)
    ));
    s.push_str(&format!(
        "        beta_g2: {},\n",
        bytes128_literal(&vk.beta_g2)
    ));
    s.push_str(&format!(
        "        gamma_g2: {},\n",
        bytes128_literal(&vk.gamma_g2)
    ));
    s.push_str(&format!(
        "        delta_g2: {},\n",
        bytes128_literal(&vk.delta_g2)
    ));

    s.push_str("        ic: vec![\n");
    for ic_point in &vk.ic {
        s.push_str(&format!("            {},\n", bytes64_literal(ic_point)));
    }
    s.push_str("        ],\n");

    s.push_str("    }\n");
    s.push_str("}\n");

    s
}

fn bytes64_literal(b: &[u8; 64]) -> String {
    format!("[\n            {}        ]",
        b.chunks(16)
            .map(|chunk| {
                chunk.iter()
                    .map(|x| format!("0x{:02x}", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .map(|line| format!("            {line},\n"))
            .collect::<String>()
    )
}

fn bytes128_literal(b: &[u8; 128]) -> String {
    format!("[\n            {}        ]",
        b.chunks(16)
            .map(|chunk| {
                chunk.iter()
                    .map(|x| format!("0x{:02x}", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .map(|line| format!("            {line},\n"))
            .collect::<String>()
    )
}
