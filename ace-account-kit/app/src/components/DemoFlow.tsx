import React, { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { SmartAccount } from "./SmartAccount";
import { KeyRotation } from "./KeyRotation";
import { OwnershipProof } from "./OwnershipProof";

type Step = "connect" | "account" | "rotate" | "ownership";

const STEPS: { key: Step; label: string; desc: string }[] = [
  { key: "connect", label: "1. Connect Wallet", desc: "Connect your Phantom or Solflare wallet" },
  { key: "account", label: "2. Smart Account", desc: "Create PDA + ZK-authorized execution" },
  { key: "rotate", label: "3. Key Rotation", desc: "Rotate to PQC key — same address" },
  { key: "ownership", label: "4. ZK-Ownership", desc: "Cross-chain proof without bridges" },
];

const STEP_EXPLAINER: Record<Step, { heading: string; what: string; why: string; points: string[] }> = {
  connect: {
    heading: "What's happening here?",
    what: "You're connecting your existing Solana wallet — the same one you already use every day.",
    why: "Nothing moves. Nothing changes. This is just the starting point.",
    points: [
      "Your money stays where it is — we never touch it",
      "No new wallet to create or address to remember",
      "Works with Phantom, Solflare, and every major Solana wallet",
    ],
  },
  account: {
    heading: "What's happening here?",
    what: "Your wallet is being upgraded into a 'smart account' — one that doesn't rely on a password or seed phrase to prove who you are.",
    why: "Instead of a key that can be stolen or guessed, your account is now protected by a mathematical proof that nobody can fake — not even a quantum computer.",
    points: [
      "Even if someone steals your device, they can't access your funds",
      "No seed phrase to lose, leak, or have phished",
      "Built to withstand threats that don't even exist yet",
    ],
  },
  rotate: {
    heading: "What's happening here?",
    what: "You're switching to a new, stronger security key — without changing your wallet address.",
    why: "Think of it like changing the lock on your front door while keeping the same house number. All your assets stay exactly where they are, and your old key is instantly deactivated.",
    points: [
      "Your old key is revoked immediately — no gap, no risk",
      "Governments are already requiring this level of security for banks and financial institutions",
      "One upgrade now vs. a costly forced migration later",
    ],
  },
  ownership: {
    heading: "What's happening here?",
    what: "You're proving that your Solana wallet also controls accounts on other blockchains — without sending a single transaction or paying any fees.",
    why: "Today, connecting wallets across blockchains requires 'bridges' — services that have been hacked for over $2.5 billion. SolAA removes the bridge entirely. You just show a proof, like showing your ID.",
    points: [
      "One identity works on Ethereum, Tron, Solana — and any future chain",
      "Claim airdrops and vote in DAOs across every chain, from one wallet",
      "No bridge means no bridge risk — instant, free, and mathematically guaranteed",
    ],
  },
};

export const DemoFlow: React.FC = () => {
  const { connected } = useWallet();
  const [currentStep, setCurrentStep] = useState<Step>("connect");
  const [idCom, setIdCom] = useState<Uint8Array | null>(null);
  const [pdaAddress, setPdaAddress] = useState<string>("");

  const explainer = STEP_EXPLAINER[currentStep];

  return (
    <div>
      {/* Step indicators */}
      <div style={styles.stepBar}>
        {STEPS.map((s) => (
          <div
            key={s.key}
            style={{
              ...styles.stepItem,
              opacity: s.key === currentStep ? 1 : 0.4,
              borderColor: s.key === currentStep ? "#22d3ee" : "#27272a",
            }}
            onClick={() => setCurrentStep(s.key)}
          >
            <div style={styles.stepLabel}>{s.label}</div>
            <div style={styles.stepDesc}>{s.desc}</div>
          </div>
        ))}
      </div>

      {/* Step content + explainer side by side */}
      <div style={styles.layout}>
        <div style={styles.main}>
          {currentStep === "connect" && (
            <div style={styles.card}>
              <h2 style={styles.cardTitle}>Connect Your Wallet</h2>
              <p style={styles.cardText}>
                Connect an existing Solana wallet to begin the zero-movement
                security upgrade. Your assets stay exactly where they are.
              </p>
              <div style={{ marginTop: "1.5rem" }}>
                <WalletMultiButton />
              </div>
              {connected && (
                <button
                  style={styles.nextBtn}
                  onClick={() => setCurrentStep("account")}
                >
                  Next →
                </button>
              )}
            </div>
          )}

          {currentStep === "account" && (
            <SmartAccount
              idCom={idCom}
              onPdaCreated={(addr) => {
                setPdaAddress(addr);
                setCurrentStep("rotate");
              }}
            />
          )}

          {currentStep === "rotate" && (
            <KeyRotation
              pdaAddress={pdaAddress}
              idCom={idCom}
              onComplete={() => setCurrentStep("ownership")}
            />
          )}

          {currentStep === "ownership" && (
            <OwnershipProof idCom={idCom} />
          )}
        </div>

        {/* Right explainer panel */}
        <div style={styles.sidebar}>
          <div style={styles.explainerBox}>
            <div style={styles.explainerHeading}>{explainer.heading}</div>
            <p style={styles.explainerWhat}>{explainer.what}</p>
            <div style={styles.whyBox}>
              <span style={styles.whyLabel}>Why it matters</span>
              <p style={styles.whyText}>{explainer.why}</p>
            </div>
            <ul style={styles.explainerList}>
              {explainer.points.map((p, i) => (
                <li key={i} style={styles.explainerItem}>
                  <span style={styles.bullet}>✓</span>
                  <span>{p}</span>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  stepBar: {
    display: "flex",
    gap: "0.5rem",
    marginBottom: "2rem",
    overflowX: "auto",
  },
  stepItem: {
    flex: 1,
    padding: "0.75rem 1rem",
    borderRadius: 8,
    border: "1px solid #27272a",
    cursor: "pointer",
    transition: "all 0.2s",
    minWidth: 160,
  },
  stepLabel: {
    fontSize: "0.85rem",
    fontWeight: 600,
    color: "#22d3ee",
  },
  stepDesc: {
    fontSize: "0.75rem",
    color: "#71717a",
    marginTop: 4,
  },
  layout: {
    display: "flex",
    gap: "1.5rem",
    alignItems: "flex-start",
  },
  main: {
    flex: "1 1 0",
    minWidth: 0,
  },
  sidebar: {
    width: 300,
    flexShrink: 0,
  },
  explainerBox: {
    background: "#18181b",
    border: "1px solid #27272a",
    borderRadius: 12,
    padding: "1.5rem",
  },
  explainerHeading: {
    fontSize: "0.7rem",
    fontWeight: 700,
    color: "#52525b",
    textTransform: "uppercase",
    letterSpacing: "0.08em",
    marginBottom: "0.75rem",
  },
  explainerWhat: {
    fontSize: "0.9rem",
    color: "#e4e4e7",
    lineHeight: 1.65,
    marginBottom: "1rem",
    fontWeight: 500,
  },
  whyBox: {
    background: "#09090b",
    border: "1px solid #27272a",
    borderRadius: 8,
    padding: "0.875rem",
    marginBottom: "1.25rem",
  },
  whyLabel: {
    display: "block",
    fontSize: "0.7rem",
    fontWeight: 700,
    color: "#22d3ee",
    textTransform: "uppercase",
    letterSpacing: "0.06em",
    marginBottom: "0.4rem",
  },
  whyText: {
    fontSize: "0.82rem",
    color: "#a1a1aa",
    lineHeight: 1.65,
    margin: 0,
  },
  explainerList: {
    listStyle: "none",
    padding: 0,
    margin: 0,
    display: "flex",
    flexDirection: "column",
    gap: "0.5rem",
  },
  explainerItem: {
    fontSize: "0.82rem",
    color: "#a1a1aa",
    lineHeight: 1.6,
    display: "flex",
    gap: "0.5rem",
    alignItems: "flex-start",
  },
  bullet: {
    color: "#22d3ee",
    fontWeight: 700,
    flexShrink: 0,
    marginTop: 1,
  },
  card: {
    background: "#18181b",
    borderRadius: 12,
    padding: "2rem",
    border: "1px solid #27272a",
  },
  cardTitle: {
    fontSize: "1.5rem",
    fontWeight: 600,
    marginBottom: "1rem",
  },
  cardText: {
    color: "#a1a1aa",
    lineHeight: 1.6,
  },
  nextBtn: {
    marginTop: "1.5rem",
    padding: "0.75rem 1.5rem",
    background: "#22d3ee",
    color: "#0a0a0f",
    border: "none",
    borderRadius: 8,
    fontWeight: 600,
    cursor: "pointer",
    fontSize: "0.95rem",
  },
};
