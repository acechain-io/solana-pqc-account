import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'SolAA — Post-Quantum Account Abstraction for Solana',
  description:
    'Upgrade your Solana wallet to post-quantum security, key rotation, and cross-chain identity — without moving a single asset. Built for Solana Frontier Hackathon 2026.',
  keywords: [
    'Solana',
    'post-quantum',
    'account abstraction',
    'ML-DSA-44',
    'ZK proof',
    'key rotation',
    'smart account',
    'PDA',
    'cross-chain identity',
    'SolAA',
  ],
  authors: [{ name: 'Jason Wang' }],
  openGraph: {
    title: 'SolAA — Post-Quantum Account Abstraction for Solana',
    description:
      'Zero-movement PQC Account Abstraction for Solana. 128-byte proofs, 52ms generation, $0.00 migration fee.',
    type: 'website',
    siteName: 'SolAA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'SolAA — Post-Quantum Account Abstraction for Solana',
    description:
      'Zero-movement PQC Account Abstraction for Solana. 128-byte proofs, 52ms generation, $0.00 migration fee.',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        <link
          href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600&display=swap"
          rel="stylesheet"
        />
      </head>
      <body className="bg-[#0a0a0f] text-slate-200 antialiased">{children}</body>
    </html>
  )
}
