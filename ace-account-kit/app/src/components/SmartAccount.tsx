import React, { useState } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";

interface Props {
  idCom: Uint8Array | null;
  onPdaCreated: (address: string) => void;
}

export const SmartAccount: React.FC<Props> = ({ idCom, onPdaCreated }) => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [status, setStatus] = useState<"idle" | "creating" | "executing" | "done">("idle");
  const [pdaAddr, setPdaAddr] = useState("");
  const [txLogs, setTxLogs] = useState<string[]>([]);

  const addLog = (msg: string) => {
    setTxLogs((prev) => [...prev, `[${new Date().toLocaleTimeString()}] ${msg}`]);
  };

  const handleCreate = async () => {
    setStatus("creating");
    addLog("Creating SolAA Smart Account PDA...");

    // Simulate PDA creation
    await new Promise((r) => setTimeout(r, 600));

    const mockPda = idCom
      ? Array.from(idCom.slice(0, 6))
          .map((b) => b.toString(16).padStart(2, "0"))
          .join("")
      : "demo";
    const addr = `PDA:${mockPda}...`;
    setPdaAddr(addr);

    addLog(`PDA created: ${addr}`);
    addLog(`  Seed: [b"solaa", id_com[0..32]]`);
    addLog(`  Domain: 2 (devnet)`);
    addLog(`  Nonce: 0`);
    addLog(`  Recovery delay: 1,512,000 slots (~7 days)`);

    setStatus("executing");
  };

  const handleExecute = async () => {
    addLog("Generating ZK-ACE proof...");
    await new Promise((r) => setTimeout(r, 200));
    addLog("  Circuit: 4,024 R1CS constraints");
    addLog("  Proving time: 52 ms");
    addLog("  Proof size: 128 bytes (Groth16)");

    addLog("Submitting ZK-authorized transaction...");
    await new Promise((r) => setTimeout(r, 400));
    addLog("  Verification: alt_bn128 pairing check");
    addLog("  CU used: ~170,500 / 200,000");
    addLog("  Result: VERIFIED");
    addLog("  Nonce bumped: 0 → 1");

    addLog("Transfer: 0.1 SOL → Alice (via CPI)");
    addLog("  No Ed25519 signature on inner payload!");

    setStatus("done");
    onPdaCreated(pdaAddr);
  };

  return (
    <div style={styles.card}>
      <h2 style={styles.title}>SolAA Smart Account</h2>
      <p style={styles.desc}>
        PDA-based account authorized by ZK proofs instead of Ed25519 signatures.
        The account address is derived from your identity commitment, not your public key.
      </p>

      <div style={styles.infoGrid}>
        <div style={styles.infoBox}>
          <span style={styles.infoLabel}>Authorization</span>
          <span style={styles.infoValue}>Groth16 ZK Proof</span>
        </div>
        <div style={styles.infoBox}>
          <span style={styles.infoLabel}>Proof Size</span>
          <span style={styles.infoValue}>128 bytes</span>
        </div>
        <div style={styles.infoBox}>
          <span style={styles.infoLabel}>On-chain CU</span>
          <span style={styles.infoValue}>~170K</span>
        </div>
        <div style={styles.infoBox}>
          <span style={styles.infoLabel}>vs ML-DSA Sig</span>
          <span style={styles.infoValue}>2,420 bytes → 128 bytes</span>
        </div>
      </div>

      <div style={styles.actions}>
        {status === "idle" && (
          <button style={styles.btn} onClick={handleCreate}>
            Create Smart Account
          </button>
        )}
        {status === "executing" && (
          <button style={styles.btn} onClick={handleExecute}>
            Execute ZK-Authorized Transfer
          </button>
        )}
        {status === "done" && (
          <button
            style={{ ...styles.btn, background: "#a78bfa" }}
            onClick={() => onPdaCreated(pdaAddr)}
          >
            Next: Key Rotation →
          </button>
        )}
      </div>

      {txLogs.length > 0 && (
        <div style={styles.logBox}>
          <div style={styles.logTitle}>Transaction Log</div>
          {txLogs.map((log, i) => (
            <div key={i} style={styles.logLine}>
              {log}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  card: { background: "#18181b", borderRadius: 12, padding: "2rem", border: "1px solid #27272a" },
  title: { fontSize: "1.5rem", fontWeight: 600, marginBottom: "0.5rem" },
  desc: { color: "#a1a1aa", marginBottom: "1.5rem", lineHeight: 1.6 },
  infoGrid: { display: "grid", gridTemplateColumns: "1fr 1fr 1fr 1fr", gap: "0.75rem", marginBottom: "1.5rem" },
  infoBox: {
    background: "#09090b", padding: "0.75rem", borderRadius: 8,
    display: "flex", flexDirection: "column",
  },
  infoLabel: { fontSize: "0.7rem", color: "#71717a", textTransform: "uppercase", letterSpacing: "0.05em" },
  infoValue: { fontSize: "0.85rem", fontWeight: 600, marginTop: 4, fontFamily: "monospace" },
  actions: { marginBottom: "1.5rem" },
  btn: {
    padding: "0.75rem 1.5rem", background: "#22d3ee", color: "#0a0a0f", border: "none",
    borderRadius: 8, fontWeight: 600, cursor: "pointer", fontSize: "0.95rem",
  },
  logBox: {
    background: "#09090b", borderRadius: 8, padding: "1rem", fontFamily: "monospace",
    fontSize: "0.8rem", maxHeight: 300, overflow: "auto",
  },
  logTitle: { color: "#22d3ee", fontWeight: 600, marginBottom: "0.5rem" },
  logLine: { color: "#a1a1aa", lineHeight: 1.6, whiteSpace: "pre-wrap" },
};
