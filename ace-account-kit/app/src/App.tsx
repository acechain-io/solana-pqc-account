import React, { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import { DemoFlow } from "./components/DemoFlow";

import "@solana/wallet-adapter-react-ui/styles.css";

const App: React.FC = () => {
  const endpoint = useMemo(() => clusterApiUrl("devnet"), []);
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          {/* Navbar */}
          <nav style={styles.navbar}>
            <a href="/" style={styles.navBrand}>
              <img src="/solaa-logo.svg" alt="SolAA" style={styles.navLogo} />
              <span style={styles.navTitle}>
                Sol<span style={styles.navTitleGradient}>AA</span>
              </span>
            </a>
            <div style={styles.navLinks}>
              <a href="/" style={styles.navLink}>Home</a>
              <a href="/#why-solaa" style={styles.navLinkWhySolaa}>Why SolAA</a>
              <a href="/#research" style={styles.navLink}>Research</a>
              <a href="/whitepaper.pdf" target="_blank" rel="noopener noreferrer" style={styles.navLinkHighlight}>Whitepaper</a>
            </div>
          </nav>

          {/* Page content */}
          <div style={styles.container}>
            <header style={styles.header}>
              <h1 style={styles.title}>SolAA</h1>
              <p style={styles.subtitle}>
                Zero-Movement PQC-Ready Account Abstraction for Solana
              </p>
            </header>
            <DemoFlow />
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

const styles: Record<string, React.CSSProperties> = {
  navbar: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "0 2rem",
    height: 56,
    borderBottom: "1px solid #27272a",
    background: "rgba(10,10,15,0.9)",
    backdropFilter: "blur(12px)",
    position: "sticky",
    top: 0,
    zIndex: 50,
  },
  navBrand: {
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",
    textDecoration: "none",
  },
  navLogo: {
    width: 28,
    height: 28,
  },
  navTitle: {
    fontWeight: 700,
    fontSize: "1.1rem",
    color: "#fff",
  },
  navTitleGradient: {
    background: "linear-gradient(135deg, #9945FF, #14F195)",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
  },
  navLinks: {
    display: "flex",
    alignItems: "center",
    gap: "2rem",
  },
  navLink: {
    color: "#94a3b8",
    textDecoration: "none",
    fontSize: "0.875rem",
    fontWeight: 500,
  },
  navLinkWhySolaa: {
    color: "#c084fc",
    textDecoration: "none",
    fontSize: "0.875rem",
    fontWeight: 700,
    padding: "0.25rem 0.75rem",
    borderRadius: 999,
    border: "1px solid rgba(153,69,255,0.4)",
    background: "rgba(153,69,255,0.15)",
  },
  navLinkHighlight: {
    color: "#c084fc",
    textDecoration: "none",
    fontSize: "0.875rem",
    fontWeight: 600,
    padding: "0.25rem 0.75rem",
    borderRadius: 6,
    border: "1px solid rgba(153,69,255,0.4)",
    background: "rgba(153,69,255,0.1)",
  },
  container: {
    maxWidth: 1280,
    margin: "0 auto",
    padding: "2rem",
  },
  header: {
    textAlign: "center",
    marginBottom: "3rem",
    borderBottom: "1px solid #27272a",
    paddingBottom: "2rem",
  },
  title: {
    fontSize: "2.5rem",
    fontWeight: 700,
    background: "linear-gradient(135deg, #22d3ee, #a78bfa)",
    WebkitBackgroundClip: "text",
    WebkitTextFillColor: "transparent",
    marginBottom: "0.5rem",
  },
  subtitle: {
    color: "#71717a",
    fontSize: "1.1rem",
  },
};

export default App;
