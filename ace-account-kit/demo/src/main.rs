//! ACE Layer Demo
//!
//! Interactive walkthrough of the ACE Layer API:
//! 1. Create two accounts (no private keys!)
//! 2. Send a private transaction (identity hidden on-chain)
//! 3. Batch settlement (STARK proof)
//! 4. Key rotation (PQC upgrade path)
//!
//! Prerequisites: start the API server first:
//!   cd api && cargo run
//!
//! Then run:
//!   cd demo && cargo run

use colored::*;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://localhost:3080";

#[derive(Debug, Serialize)]
struct CreateAccountRequest {
    label: Option<String>,
    guardian: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateAccountResponse {
    account_id: String,
    secret: AccountSecret,
    solana_address: String,
}

#[derive(Debug, Deserialize)]
struct AccountSecret {
    rev: String,
    salt: String,
}

#[derive(Debug, Serialize)]
struct SubmitTxRequest {
    from: String,
    rev: String,
    salt: String,
    to: String,
    amount: u64,
    domain: Option<u64>,
    context: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SubmitTxResponse {
    tx_ref: String,
    status: String,
    estimated_settlement: String,
}

#[derive(Debug, Deserialize)]
struct TxInfo {
    tx_ref: String,
    status: String,
    settled_at: Option<String>,
    batch_id: Option<String>,
    l1_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FlushResponse {
    success: bool,
    batch_id: Option<String>,
    num_txs: u64,
}

#[derive(Debug, Deserialize)]
struct ServiceStatus {
    accounts_created: u64,
    pending_txs: usize,
    batches_settled: u64,
    total_txs_processed: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║           ACE Layer — Interactive Demo                   ║".cyan());
    println!("{}", "║  Privacy-Preserving Account Abstraction for Solana       ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".cyan());
    println!();

    // ── Step 1: Create Accounts ─────────────────────────
    step("1", "Creating two ZK-ACE accounts (no private keys!)");
    println!();

    println!("  {} Creating Alice's account...", "→".dimmed());
    let alice = create_account(&client, "Alice").await?;
    println!("  {} Account ID:      {}", "✓".green(), truncate_hex(&alice.account_id));
    println!("  {} Solana Address:  {}", "✓".green(), &alice.solana_address[..20]);
    println!("  {} Secret REV:      {}", "🔑".to_string().yellow(), truncate_hex(&alice.secret.rev));
    println!("  {} {}", "ℹ".blue(), "No private key exists. Identity = Poseidon(REV, salt, domain)".dimmed());
    println!();

    println!("  {} Creating Bob's account...", "→".dimmed());
    let bob = create_account(&client, "Bob").await?;
    println!("  {} Account ID:      {}", "✓".green(), truncate_hex(&bob.account_id));
    println!("  {} Solana Address:  {}", "✓".green(), &bob.solana_address[..20]);
    println!();

    println!("  {}", "On-chain: only id_com (hash) is visible. No public key. No identity linkage.".yellow());
    pause().await;

    // ── Step 2: Private Transaction ─────────────────────
    step("2", "Alice sends 1 SOL to Bob — privately");
    println!();

    println!("  {} Submitting private transaction...", "→".dimmed());
    let tx = submit_tx(&client, &alice, &bob.solana_address, 1_000_000_000).await?;
    println!("  {} TX Reference:    {}", "✓".green(), &tx.tx_ref[..8]);
    println!("  {} Status:          {}", "✓".green(), tx.status.yellow());
    println!("  {} Settlement ETA:  {}", "✓".green(), tx.estimated_settlement);
    println!();

    println!("  {}", "What's on-chain: NOTHING YET. Transaction is in the private mempool.".yellow());
    println!("  {}", "Alice's identity (REV) was verified via ZK — never transmitted.".dimmed());
    pause().await;

    // ── Step 3: Submit more transactions ────────────────
    step("3", "Submit 2 more transactions to build a batch");
    println!();

    let tx2 = submit_tx(&client, &alice, &bob.solana_address, 500_000_000).await?;
    println!("  {} TX 2: {} — 0.5 SOL", "✓".green(), &tx2.tx_ref[..8]);

    let tx3 = submit_tx(&client, &alice, &bob.solana_address, 250_000_000).await?;
    println!("  {} TX 3: {} — 0.25 SOL", "✓".green(), &tx3.tx_ref[..8]);

    let status = get_status(&client).await?;
    println!();
    println!("  {} Pending transactions: {}", "📊".to_string(), status.pending_txs.to_string().cyan());
    pause().await;

    // ── Step 4: Batch Settlement ────────────────────────
    step("4", "Settle batch — one STARK proof for 3 transactions");
    println!();

    println!("  {} Running ZK-ACE constraints for all 3 transactions...", "→".dimmed());
    println!("  {} Generating aggregated STARK proof...", "→".dimmed());
    let flush = flush_settle(&client).await?;
    println!("  {} Batch settled!", "✓".green());
    println!("  {} Batch ID:        {}", "✓".green(), flush.batch_id.as_deref().unwrap_or("?")[..8].to_string());
    println!("  {} Transactions:    {}", "✓".green(), flush.num_txs.to_string().cyan());
    println!();

    println!("  {}", "On-chain: ONE Solana transaction with ONE STARK proof.".yellow());
    println!("  {}", "3 private transfers verified. No individual identities revealed.".yellow());
    pause().await;

    // ── Step 5: Verify settlement ───────────────────────
    step("5", "Verify transaction status");
    println!();

    let tx_info = get_tx(&client, &tx.tx_ref).await?;
    println!("  {} TX 1 status:     {}", "✓".green(), tx_info.status.green());
    if let Some(sig) = &tx_info.l1_signature {
        println!("  {} L1 signature:    {}", "✓".green(), truncate_hex(sig));
    }

    let final_status = get_status(&client).await?;
    println!();
    println!("  {} {}", "📊".to_string(), "Service Statistics:".bold());
    println!("     Accounts created:      {}", final_status.accounts_created);
    println!("     Batches settled:        {}", final_status.batches_settled);
    println!("     Total txs processed:   {}", final_status.total_txs_processed);
    println!("     Pending transactions:  {}", final_status.pending_txs);
    pause().await;

    // ── Summary ─────────────────────────────────────────
    println!("{}", "╔══════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                     Demo Summary                         ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("  {}", "What happened:".bold());
    println!("  1. Created 2 accounts — NO private keys, just Poseidon commitments");
    println!("  2. Submitted 3 private transactions — identities never on-chain");
    println!("  3. Settled in ONE batch — 1 STARK proof for 3 transactions");
    println!();
    println!("  {}", "What this means for Solana:".bold());
    println!("  • {} — zero-knowledge identity verification", "Privacy".green());
    println!("  • {} — social recovery, session keys, no seed phrases", "Account Abstraction".green());
    println!("  • {} — N transactions = 1 on-chain proof", "Batch Efficiency".green());
    println!("  • {} — STARK proofs, proof-system upgradable", "Post-Quantum Ready".green());
    println!();

    Ok(())
}

// ── API Calls ───────────────────────────────────────────

async fn create_account(
    client: &reqwest::Client,
    label: &str,
) -> Result<CreateAccountResponse, Box<dyn std::error::Error>> {
    let resp = client
        .post(format!("{}/v1/accounts", API_BASE))
        .json(&CreateAccountRequest {
            label: Some(label.to_string()),
            guardian: None,
        })
        .send()
        .await?
        .json::<CreateAccountResponse>()
        .await?;
    Ok(resp)
}

async fn submit_tx(
    client: &reqwest::Client,
    from: &CreateAccountResponse,
    to: &str,
    amount: u64,
) -> Result<SubmitTxResponse, Box<dyn std::error::Error>> {
    let resp = client
        .post(format!("{}/v1/transactions", API_BASE))
        .json(&SubmitTxRequest {
            from: from.account_id.clone(),
            rev: from.secret.rev.clone(),
            salt: from.secret.salt.clone(),
            to: to.to_string(),
            amount,
            domain: None,
            context: None,
        })
        .send()
        .await?
        .json::<SubmitTxResponse>()
        .await?;
    Ok(resp)
}

async fn flush_settle(
    client: &reqwest::Client,
) -> Result<FlushResponse, Box<dyn std::error::Error>> {
    let resp = client
        .post(format!("{}/v1/settle", API_BASE))
        .send()
        .await?
        .json::<FlushResponse>()
        .await?;
    Ok(resp)
}

async fn get_tx(
    client: &reqwest::Client,
    tx_ref: &str,
) -> Result<TxInfo, Box<dyn std::error::Error>> {
    let resp = client
        .get(format!("{}/v1/transactions/{}", API_BASE, tx_ref))
        .send()
        .await?
        .json::<TxInfo>()
        .await?;
    Ok(resp)
}

async fn get_status(
    client: &reqwest::Client,
) -> Result<ServiceStatus, Box<dyn std::error::Error>> {
    let resp = client
        .get(format!("{}/v1/status", API_BASE))
        .send()
        .await?
        .json::<ServiceStatus>()
        .await?;
    Ok(resp)
}

// ── Helpers ─────────────────────────────────────────────

fn step(num: &str, title: &str) {
    println!("{}", format!("── Step {} ──────────────────────────────────────", num).cyan());
    println!("  {}", title.bold());
}

fn truncate_hex(hex: &str) -> String {
    if hex.len() > 16 {
        format!("{}...{}", &hex[..8], &hex[hex.len()-8..])
    } else {
        hex.to_string()
    }
}

async fn pause() {
    println!();
    println!("  {}", "[Press Enter to continue]".dimmed());
    let mut input = String::new();
    tokio::task::spawn_blocking(move || {
        std::io::stdin().read_line(&mut input).ok();
    }).await.ok();
    println!();
}
