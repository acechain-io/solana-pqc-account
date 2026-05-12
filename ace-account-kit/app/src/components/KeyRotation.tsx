import React, { useState } from "react";

interface Props {
  pdaAddress: string;
  idCom: Uint8Array | null;
  onComplete: () => void;
}

export const KeyRotation: React.FC<Props> = ({ pdaAddress, idCom, onComplete }) => {
  const [phase, setPhase] = useState<"explain" | "rotating" | "done">("explain");
  const [logs, setLogs] = useState<string[]>([]);

  const addLog = (msg: string) => {
    setLogs((prev) => [...prev, msg]);
  };

  const handleRotate = async () => {
    setPhase("rotating");

    addLog("Step 1: Generate ZK proof of current key ownership...");
    await new Promise((r) => setTimeout(r, 300));
    addLog("  Proving: Poseidon(REV_old, salt, domain) == id_com_current");
    addLog("  Proof generated: 128 bytes, 52 ms");

    addLog("");
    addLog("Step 2: Derive new identity commitment (ML-DSA-44)...");
    await new Promise((r) => setTimeout(r, 200));
    addLog("  New key type: ML-DSA-44 (NIST PQC standard)");
    addLog("  new_id_com = Poseidon(REV_new, salt_new, domain)");

    addLog("");
    addLog("Step 3: Submit rotate_key transaction...");
    await new Promise((r) => setTimeout(r, 400));
    addLog("  On-chain Groth16 verification: PASSED");
    addLog("  id_com updated: old → new");
    addLog("  Nonce bumped: 1 → 2");
    addLog("  Pending recovery cleared");

    addLog("");
    addLog("RESULT:");
    addLog(`  PDA address: ${pdaAddress} (UNCHANGED)`);
    addLog("  All assets: UNMOVED");
    addLog("  Auth scheme: Ed25519 → ML-DSA-44 (PQC)");
    addLog("  Quantum resistance: ACTIVE");

    setPhase("done");
  };

  return (
    <div style={styles.card}>
      <h2 style={styles.title}>Key Rotation — The Killer Feature</h2>

      {phase === "explain" && (
        <>
          <div style={styles.comparison}>
            <div style={styles.compBox}>
              <h3 style={styles.compTitle}>Traditional Solana</h3>
              <p style={styles.compText}>
                <code>address = Ed25519_pubkey</code>
              </p>
              <p style={styles.compText}>
                Change key = Change address = Move ALL assets
              </p>
              <p style={styles.compCost}>Cost: $50–200+, hours of work</p>
            </div>
            <div style={{ ...styles.compBox, borderColor: "#166534", background: "#052e16" }}>
              <h3 style={styles.compTitle}>SolAA Smart Account</h3>
              <p style={styles.compText}>
                <code>PDA = f(program_id, seed)</code>
              </p>
              <p style={styles.compText}>
                rotate_key updates id_com, PDA address unchanged
              </p>
              <p style={{ ...styles.compCost, color: "#22d3ee" }}>
                Cost: 1 transaction (~$0.001)
              </p>
            </div>
          </div>

          <h3 style={styles.subtitle}>What this enables:</h3>
          <ul style={styles.list}>
            <li><strong>Ed25519 → ML-DSA-44</strong> — Post-quantum upgrade</li>
            <li><strong>Compromised key → Fresh key</strong> — Emergency rotation</li>
            <li><strong>Single-sig → Multi-sig</strong> — Security upgrade</li>
          </ul>

          <button style={styles.btn} onClick={handleRotate}>
            Simulate PQC Key Rotation
          </button>
        </>
      )}

      {(phase === "rotating" || phase === "done") && (
        <div style={styles.logBox}>
          {logs.map((log, i) => (
            <div
              key={i}
              style={{
                ...styles.logLine,
                color: log.startsWith("RESULT") ? "#22d3ee" : "#a1a1aa",
                fontWeight: log.startsWith("RESULT") ? 700 : 400,
              }}
            >
              {log}
            </div>
          ))}
        </div>
      )}

      {phase === "done" && (
        <button
          style={{ ...styles.btn, marginTop: "1rem", background: "#a78bfa" }}
          onClick={onComplete}
        >
          Next: Cross-Chain Ownership →
        </button>
      )}
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  card: { background: "#18181b", borderRadius: 12, padding: "2rem", border: "1px solid #27272a" },
  title: { fontSize: "1.5rem", fontWeight: 600, marginBottom: "1rem" },
  subtitle: { fontSize: "1.1rem", fontWeight: 600, margin: "1.5rem 0 0.5rem" },
  comparison: { display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1rem", marginBottom: "1.5rem" },
  compBox: { padding: "1rem", borderRadius: 8, border: "1px solid #44403c", background: "#1c1917" },
  compTitle: { fontSize: "0.95rem", fontWeight: 600, marginBottom: "0.5rem" },
  compText: { fontSize: "0.85rem", color: "#a1a1aa", marginBottom: 4, lineHeight: 1.5 },
  compCost: { fontSize: "0.85rem", color: "#ef4444", fontWeight: 600, marginTop: 8 },
  list: { listStyle: "disc", paddingLeft: "1.5rem", color: "#a1a1aa", lineHeight: 2, marginBottom: "1.5rem" },
  btn: {
    padding: "0.75rem 1.5rem", background: "#22d3ee", color: "#0a0a0f", border: "none",
    borderRadius: 8, fontWeight: 600, cursor: "pointer", fontSize: "0.95rem",
  },
  logBox: {
    background: "#09090b", borderRadius: 8, padding: "1rem", fontFamily: "monospace",
    fontSize: "0.8rem", maxHeight: 400, overflow: "auto",
  },
  logLine: { color: "#a1a1aa", lineHeight: 1.6, whiteSpace: "pre-wrap" },
};
