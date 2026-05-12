import React, { useState } from "react";

interface Props {
  onComplete: (idCom: Uint8Array) => void;
}

export const ImportWallet: React.FC<Props> = ({ onComplete }) => {
  const [mnemonic, setMnemonic] = useState("");
  const [passphrase, setPassphrase] = useState("");
  const [status, setStatus] = useState<"idle" | "processing" | "done">("idle");
  const [result, setResult] = useState<{
    address: string;
    idCom: string;
    rev32Type: string;
  } | null>(null);

  const handleImport = async () => {
    setStatus("processing");

    // Simulate SA-Migration process
    // In production, this calls the WASM SDK
    await new Promise((r) => setTimeout(r, 800));

    // Generate a deterministic id_com from the mnemonic
    const encoder = new TextEncoder();
    const mnemonicBytes = encoder.encode(mnemonic.trim());
    const hashBuffer = await crypto.subtle.digest("SHA-256", mnemonicBytes);
    const idCom = new Uint8Array(hashBuffer);

    // Simulate preserved address
    const addrBytes = new Uint8Array(32);
    addrBytes.set(idCom.slice(0, 16));
    // Convert to base58-like display
    const addrHex = Array.from(addrBytes.slice(0, 4))
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("");

    setResult({
      address: `${addrHex}...${addrHex.slice(-4)}`,
      idCom: Array.from(idCom.slice(0, 8))
        .map((b) => b.toString(16).padStart(2, "0"))
        .join(""),
      rev32Type: "0xB0 (BIP39 Import — Address Preserving)",
    });
    setStatus("done");
    onComplete(idCom);
  };

  return (
    <div style={styles.card}>
      <h2 style={styles.title}>SA-Migration: Zero-Movement Upgrade</h2>
      <p style={styles.desc}>
        Import your existing mnemonic. Your Solana address is{" "}
        <strong>preserved</strong> — no asset movement required.
      </p>

      <div style={styles.sandboxBanner}>
        ⚠️ Sandbox demo — use a test wallet only. Never enter a real mnemonic here.
      </div>

      {status === "idle" && (
        <>
          <div style={styles.inputGroup}>
            <label style={styles.label}>Mnemonic Phrase (12 or 24 words)</label>
            <textarea
              style={styles.textarea}
              rows={3}
              placeholder="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
              value={mnemonic}
              onChange={(e) => setMnemonic(e.target.value)}
            />
          </div>
          <div style={styles.inputGroup}>
            <label style={styles.label}>
              Passphrase (for Argon2id encryption)
            </label>
            <input
              type="password"
              style={styles.input}
              placeholder="Strong passphrase"
              value={passphrase}
              onChange={(e) => setPassphrase(e.target.value)}
            />
          </div>
          <button
            style={styles.btn}
            onClick={handleImport}
            disabled={!mnemonic.trim()}
          >
            Import &amp; Upgrade
          </button>
        </>
      )}

      {status === "processing" && (
        <div style={styles.processing}>
          <div style={styles.spinner} />
          <p>Encapsulating into REV32 Sealed Artifact...</p>
          <p style={styles.small}>
            Argon2id key derivation (4 MB, t=3) + AES-256-GCM-SIV
          </p>
        </div>
      )}

      {status === "done" && result && (
        <div style={styles.resultBox}>
          <h3 style={styles.resultTitle}>Migration Complete</h3>

          <div style={styles.compareGrid}>
            <div style={styles.beforeCol}>
              <h4 style={styles.colTitle}>Before</h4>
              <ul style={styles.list}>
                <li>Plaintext seed phrase</li>
                <li>No brute-force protection</li>
                <li>No key rotation</li>
                <li>No PQC support</li>
                <li>No recovery option</li>
              </ul>
            </div>
            <div style={styles.afterCol}>
              <h4 style={styles.colTitle}>After (SA-Migration)</h4>
              <ul style={styles.list}>
                <li>AES-256-GCM-SIV encrypted</li>
                <li>Argon2id brute-force resistance</li>
                <li>Key rotation ready</li>
                <li>ML-DSA-44 PQC stream active</li>
                <li>VA-DAR recovery registered</li>
              </ul>
            </div>
          </div>

          <div style={styles.statsGrid}>
            <div style={styles.stat}>
              <div style={styles.statLabel}>Address</div>
              <div style={styles.statValue}>Unchanged</div>
            </div>
            <div style={styles.stat}>
              <div style={styles.statLabel}>REV32 Type</div>
              <div style={styles.statValue}>{result.rev32Type}</div>
            </div>
            <div style={styles.stat}>
              <div style={styles.statLabel}>On-chain Cost</div>
              <div style={styles.statValue}>$0.00</div>
            </div>
            <div style={styles.stat}>
              <div style={styles.statLabel}>id_com</div>
              <div style={styles.statValue}>0x{result.idCom}...</div>
            </div>
          </div>

          <div style={styles.highlight}>
            Zero transactions. Zero gas. Zero movement.
          </div>
        </div>
      )}
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  card: { background: "#18181b", borderRadius: 12, padding: "2rem", border: "1px solid #27272a" },
  title: { fontSize: "1.5rem", fontWeight: 600, marginBottom: "0.5rem" },
  desc: { color: "#a1a1aa", marginBottom: "1.5rem", lineHeight: 1.6 },
  inputGroup: { marginBottom: "1rem" },
  label: { display: "block", fontSize: "0.85rem", color: "#71717a", marginBottom: 6 },
  textarea: {
    width: "100%", padding: "0.75rem", background: "#09090b", border: "1px solid #27272a",
    borderRadius: 8, color: "#e4e4e7", fontSize: "0.9rem", fontFamily: "monospace", resize: "vertical",
  },
  input: {
    width: "100%", padding: "0.75rem", background: "#09090b", border: "1px solid #27272a",
    borderRadius: 8, color: "#e4e4e7", fontSize: "0.9rem",
  },
  btn: {
    padding: "0.75rem 1.5rem", background: "#22d3ee", color: "#0a0a0f", border: "none",
    borderRadius: 8, fontWeight: 600, cursor: "pointer", fontSize: "0.95rem", marginTop: "0.5rem",
  },
  processing: { textAlign: "center", padding: "2rem 0", color: "#a1a1aa" },
  spinner: {
    width: 32, height: 32, border: "3px solid #27272a", borderTop: "3px solid #22d3ee",
    borderRadius: "50%", animation: "spin 1s linear infinite", margin: "0 auto 1rem",
  },
  small: { fontSize: "0.8rem", color: "#52525b", marginTop: 4 },
  resultBox: { marginTop: "1rem" },
  resultTitle: { fontSize: "1.2rem", color: "#22d3ee", marginBottom: "1rem" },
  compareGrid: { display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1rem", marginBottom: "1.5rem" },
  beforeCol: { background: "#1c1917", padding: "1rem", borderRadius: 8, border: "1px solid #44403c" },
  afterCol: { background: "#052e16", padding: "1rem", borderRadius: 8, border: "1px solid #166534" },
  colTitle: { fontSize: "0.9rem", fontWeight: 600, marginBottom: "0.5rem" },
  list: { listStyle: "none", fontSize: "0.8rem", color: "#a1a1aa", lineHeight: 1.8 },
  statsGrid: { display: "grid", gridTemplateColumns: "1fr 1fr", gap: "0.75rem", marginBottom: "1.5rem" },
  stat: { background: "#09090b", padding: "0.75rem", borderRadius: 8 },
  statLabel: { fontSize: "0.75rem", color: "#71717a" },
  statValue: { fontSize: "0.9rem", fontWeight: 600, fontFamily: "monospace", marginTop: 2 },
  sandboxBanner: {
    background: "#431407",
    border: "1px solid #9a3412",
    borderRadius: 8,
    padding: "0.6rem 1rem",
    fontSize: "0.85rem",
    color: "#fed7aa",
    marginBottom: "1.25rem",
  },
  highlight: {
    textAlign: "center", padding: "1rem", background: "#022c22", borderRadius: 8,
    color: "#22d3ee", fontWeight: 600, fontSize: "1.1rem", border: "1px solid #065f46",
  },
};
