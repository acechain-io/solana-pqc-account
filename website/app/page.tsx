'use client'

import { useState, useEffect } from 'react'

// ─── Icons (inline SVG to avoid external dependencies) ────────────────────────

function IconShield({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z" />
    </svg>
  )
}

function IconKey({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z" />
    </svg>
  )
}

function IconGlobe({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0112 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 013 12c0-1.605.42-3.113 1.157-4.418" />
    </svg>
  )
}

function IconZap({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
    </svg>
  )
}

function IconCheck({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
    </svg>
  )
}

function IconX({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  )
}

function IconGithub({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
    </svg>
  )
}

function IconDocument({ className = '' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
    </svg>
  )
}

// ─── Live Demo Section ───────────────────────────────────��────────────────────

const DEMO_ADDRESS = '7nWq4sJMmqVt3xRBBHFRdWnE2cqk1YhzPUNaMLBpHgEf'
const DEMO_ID_COM   = 'cf15a0e5b4b3a0b327f7fcc66b2139638875ae1577a9d9735ee65358b10b7be8'
const DEMO_PROOF_EXCERPT = 'cf15a0e5...7be8 • 1,842 bytes • Circle STARK'
const DEMO_TX_SIG   = '4xKzV9...mRnP'

type DemoStep = 0 | 1 | 2 | 3 | 4

interface StepState {
  done: boolean
  active: boolean
  log?: string
}

function LiveDemoSection() {
  const [step, setStep] = useState<DemoStep>(0)
  const [running, setRunning] = useState(false)
  const [logs, setLogs] = useState<string[]>([])

  const addLog = (msg: string) =>
    setLogs(prev => [...prev, `> ${msg}`])

  const runStep = async (s: DemoStep) => {
    if (running) return
    setRunning(true)

    const delays: Record<DemoStep, [string[], number[]]> = {
      0: [
        ['Importing BIP39 mnemonic (12 words)…'],
        [600],
      ],
      1: [
        [
          'Extracting 128-bit BIP39 entropy…',
          'Packing into REV32 format (v=0xB0 BIP39 import)…',
          'KDF: Argon2id m=4096KB t=3 p=1…',
          'Encrypting with AES-256-GCM-SIV…',
          `✓ Sealed Artifact created — address preserved: ${DEMO_ADDRESS.slice(0, 16)}…`,
        ],
        [300, 400, 700, 400, 500],
      ],
      2: [
        [
          `Computing id_com = SHA-256(REV ‖ salt ‖ domain)…`,
          `id_com = ${DEMO_ID_COM.slice(0, 20)}…`,
          'Deriving PDA: [b"ace-aa", id_com]…',
          '✓ SolAA Smart Account initialized on Solana devnet',
          'Nonce: 0   Domain: 1   Guardian: none',
        ],
        [300, 200, 400, 600, 200],
      ],
      3: [
        [
          'Payload: transfer 1.5 SOL to 3fKz…',
          'TxHash = SHA-256(payload)…',
          'Generating Circle STARK proof (Stwo, post-quantum)…',
          `Proof size: 1,842 bytes — post-quantum secure ✓`,
          'Submitting ProofData::Stark to Solana devnet…',
          `✓ Transaction confirmed: ${DEMO_TX_SIG}   CU: 198,402`,
        ],
        [200, 200, 900, 300, 500, 600],
      ],
      4: [
        [
          'ZK-proving ownership of current identity…',
          'Deriving ML-DSA-44 key stream from REV (HKDF #7)…',
          'new_id_com = SHA-256(REV ‖ new_salt ‖ domain)…',
          'Submitting rotate_key(new_id_com, proof)…',
          `✓ Key rotated — same PDA address: ${DEMO_ADDRESS.slice(0, 16)}…`,
          '✓ Post-quantum ML-DSA-44 active — quantum-resistant now',
        ],
        [300, 400, 300, 600, 400, 400],
      ],
    }

    const [msgs, waitMs] = delays[s]
    for (let i = 0; i < msgs.length; i++) {
      await new Promise(r => setTimeout(r, i === 0 ? 100 : waitMs[i - 1] ?? 400))
      addLog(msgs[i])
    }

    setStep((s + 1) as DemoStep)
    setRunning(false)
  }

  const reset = () => {
    setStep(0)
    setLogs([])
    setRunning(false)
  }

  const steps = [
    { label: '1. Import Wallet', sublabel: 'SA-Migration' },
    { label: '2. Seal & Upgrade', sublabel: 'Argon2id + AES-GCM-SIV' },
    { label: '3. Create Smart Account', sublabel: 'On-chain PDA' },
    { label: '4. STARK Transfer', sublabel: 'Circle STARK proof' },
    { label: '5. PQC Key Rotation', sublabel: 'Ed25519 → ML-DSA-44' },
  ]

  const doneAll = step >= 5

  return (
    <section id="demo" className="py-24 relative">
      {/* background */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-[#9945FF]/3 to-transparent pointer-events-none" />

      <div className="max-w-6xl mx-auto px-6">
        {/* header */}
        <div className="text-center mb-14">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-[#14F195]/10 border border-[#14F195]/20 text-[#14F195] text-xs font-mono mb-4">
            <span className="w-1.5 h-1.5 rounded-full bg-[#14F195] animate-pulse" />
            LIVE INTERACTIVE DEMO
          </div>
          <h2 className="text-3xl sm:text-4xl font-bold text-white mb-4">
            Quantum-Safe DeFi in 30 Seconds
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            Watch how SolAA upgrades an existing Phantom wallet to post-quantum
            security — without moving a single token. Click each step to run it.
          </p>
        </div>

        <div className="grid lg:grid-cols-5 gap-4 mb-8">
          {/* step buttons */}
          {steps.map((s, i) => {
            const isDone = step > i
            const isActive = step === i
            return (
              <button
                key={i}
                onClick={() => isActive && !running ? runStep(i as DemoStep) : undefined}
                disabled={!isActive || running}
                className={`relative p-4 rounded-xl border text-left transition-all duration-300 ${
                  isDone
                    ? 'border-[#14F195]/40 bg-[#14F195]/5'
                    : isActive
                    ? 'border-[#9945FF] bg-[#9945FF]/10 shadow-[0_0_20px_rgba(153,69,255,0.2)] cursor-pointer hover:bg-[#9945FF]/15'
                    : 'border-slate-800 bg-slate-900/40 opacity-40 cursor-not-allowed'
                }`}
              >
                <div className={`w-6 h-6 rounded-full flex items-center justify-center mb-2 text-xs font-bold ${
                  isDone ? 'bg-[#14F195] text-black' : isActive ? 'bg-[#9945FF] text-white' : 'bg-slate-700 text-slate-400'
                }`}>
                  {isDone ? '✓' : i + 1}
                </div>
                <div className={`text-xs font-semibold mb-0.5 ${isDone ? 'text-[#14F195]' : isActive ? 'text-white' : 'text-slate-500'}`}>
                  {s.label}
                </div>
                <div className="text-[10px] text-slate-600 font-mono">{s.sublabel}</div>
                {isActive && !running && (
                  <div className="absolute inset-x-0 bottom-0 h-0.5 bg-gradient-to-r from-[#9945FF] to-[#14F195] rounded-b-xl" />
                )}
              </button>
            )
          })}
        </div>

        {/* terminal */}
        <div className="rounded-2xl border border-slate-700/60 bg-[#0d0d14] overflow-hidden shadow-2xl">
          {/* terminal title bar */}
          <div className="flex items-center gap-2 px-4 py-3 bg-slate-800/60 border-b border-slate-700/60">
            <div className="w-3 h-3 rounded-full bg-red-500/80" />
            <div className="w-3 h-3 rounded-full bg-yellow-500/80" />
            <div className="w-3 h-3 rounded-full bg-green-500/80" />
            <span className="ml-3 text-slate-500 text-xs font-mono">solaa — devnet</span>
            <div className="ml-auto flex items-center gap-1.5">
              <span className="w-1.5 h-1.5 rounded-full bg-[#14F195] animate-pulse" />
              <span className="text-[#14F195] text-[10px] font-mono">CONNECTED</span>
            </div>
          </div>

          {/* log area */}
          <div className="p-5 font-mono text-sm min-h-[220px] max-h-[340px] overflow-y-auto">
            {logs.length === 0 && (
              <p className="text-slate-600">
                {`// Click "1. Import Wallet" to begin the demo`}
              </p>
            )}
            {logs.map((line, i) => (
              <div
                key={i}
                className={`leading-relaxed ${
                  line.includes('✓')
                    ? 'text-[#14F195]'
                    : line.startsWith('> Generating') || line.startsWith('> ZK-')
                    ? 'text-[#9945FF]'
                    : 'text-slate-300'
                }`}
              >
                {line}
              </div>
            ))}
            {running && (
              <div className="text-[#9945FF] animate-pulse">{'> …'}</div>
            )}
          </div>

          {/* status bar */}
          <div className="px-5 py-3 bg-slate-800/30 border-t border-slate-700/40 flex items-center justify-between gap-4 text-[11px] font-mono flex-wrap">
            <div className="flex items-center gap-4 text-slate-500">
              <span>address: <span className="text-slate-300">{step >= 2 ? DEMO_ADDRESS.slice(0, 20) + '…' : '—'}</span></span>
              <span>proof: <span className="text-[#9945FF]">{step >= 4 ? DEMO_PROOF_EXCERPT : '—'}</span></span>
              <span>nonce: <span className="text-slate-300">{step >= 4 ? '1' : step >= 3 ? '1' : '0'}</span></span>
            </div>
            <div className="flex items-center gap-2 text-slate-500">
              {step >= 5 ? (
                <span className="text-[#14F195]">pqc: ML-DSA-44 active ✓</span>
              ) : (
                <span>pqc: upgrading…</span>
              )}
            </div>
          </div>
        </div>

        {/* completion banner */}
        {doneAll && (
          <div className="mt-6 p-5 rounded-xl border border-[#14F195]/30 bg-[#14F195]/5 flex items-start gap-4">
            <div className="w-10 h-10 rounded-full bg-[#14F195]/20 flex items-center justify-center flex-shrink-0 mt-0.5">
              <IconShield className="w-5 h-5 text-[#14F195]" />
            </div>
            <div>
              <p className="text-white font-semibold mb-1">
                Upgrade complete — 0 transactions, 0 gas for migration
              </p>
              <p className="text-slate-400 text-sm">
                Alice's wallet is now protected by Circle STARK (post-quantum). The same address,
                the same assets, the same DeFi positions — now quantum-safe and able to rotate
                keys in milliseconds for $0.001.
              </p>
              <button
                onClick={reset}
                className="mt-3 text-xs text-slate-500 hover:text-white underline transition-colors"
              >
                Reset demo →
              </button>
            </div>
          </div>
        )}

        {/* before/after comparison */}
        <div className="mt-10 grid sm:grid-cols-2 gap-4">
          {[
            {
              title: 'Before SolAA',
              color: 'red',
              items: [
                'Ed25519 key — quantum-vulnerable',
                'Key rotation = migrate all assets ($150+)',
                'No social recovery',
                'Cross-chain identity requires bridges',
                'Single point of failure: seed phrase',
              ],
            },
            {
              title: 'After SolAA',
              color: 'green',
              items: [
                'Circle STARK authorization — post-quantum ✓',
                'Key rotation = 1 tx, $0.001, same address ✓',
                'Social recovery with 7-day timelock ✓',
                'Cross-chain ZK-Ownership (no bridge) ✓',
                'Sealed Artifact: Argon2id + AES-256-GCM-SIV ✓',
              ],
            },
          ].map(({ title, color, items }) => (
            <div
              key={title}
              className={`p-5 rounded-xl border ${
                color === 'red'
                  ? 'border-red-900/40 bg-red-950/10'
                  : 'border-[#14F195]/30 bg-[#14F195]/5'
              }`}
            >
              <h4 className={`text-sm font-semibold mb-3 ${color === 'red' ? 'text-red-400' : 'text-[#14F195]'}`}>
                {title}
              </h4>
              <ul className="space-y-2">
                {items.map((item, i) => (
                  <li key={i} className="flex items-start gap-2 text-sm text-slate-400">
                    <span className={`mt-0.5 flex-shrink-0 ${color === 'red' ? 'text-red-500' : 'text-[#14F195]'}`}>
                      {color === 'red' ? '✗' : '✓'}
                    </span>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Navbar ───────────────────────────────────────────────────────────────────

function Navbar() {
  const [scrolled, setScrolled] = useState(false)
  const [menuOpen, setMenuOpen] = useState(false)

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 20)
    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [])

  return (
    <nav
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
        scrolled
          ? 'bg-[#0a0a0f]/90 backdrop-blur-xl border-b border-[#9945FF]/20 shadow-[0_4px_30px_rgba(153,69,255,0.1)]'
          : 'bg-transparent'
      }`}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <a href="#" className="flex items-center gap-2.5 group">
            <img src="/solaa-logo.svg" alt="SolAA" className="w-8 h-8 drop-shadow-[0_0_8px_rgba(153,69,255,0.6)]" />
            <span className="font-bold text-white text-lg tracking-tight">
              Sol<span className="gradient-text">AA</span>
            </span>
          </a>

          {/* Desktop nav */}
          <div className="hidden md:flex items-center gap-8">
            <a
              href="#why-solaa"
              className="text-sm font-bold px-3 py-1 rounded-full transition-all duration-200"
              style={{ background: 'rgba(153,69,255,0.15)', color: '#c084fc', border: '1px solid rgba(153,69,255,0.4)' }}
            >
              Why SolAA
            </a>
            {[
              { label: 'Features', href: '#solution' },
              { label: 'How it Works', href: '#how-it-works' },
              { label: 'Demo', href: '#demo' },
              { label: 'Research', href: '#research' },
              { label: 'Team', href: '#team' },
            ].map((link) => (
              <a
                key={link.label}
                href={link.href}
                className="text-slate-400 hover:text-white text-sm font-medium transition-colors duration-200 hover:text-[#14F195]"
              >
                {link.label}
              </a>
            ))}
          </div>

          {/* CTA */}
          <div className="hidden md:flex items-center gap-3">
            <a
              href="/demo/"
              className="px-5 py-2 rounded-lg bg-gradient-to-r from-[#9945FF] to-[#7c3aed] text-white text-sm font-semibold hover:shadow-[0_0_20px_rgba(153,69,255,0.5)] transition-all duration-200 hover:-translate-y-0.5"
            >
              Try Demo →
            </a>
          </div>

          {/* Mobile menu button */}
          <button
            onClick={() => setMenuOpen(!menuOpen)}
            className="md:hidden p-2 rounded-lg text-slate-400 hover:text-white hover:bg-white/5 transition-colors"
            aria-label="Toggle menu"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              {menuOpen ? (
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              ) : (
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
              )}
            </svg>
          </button>
        </div>
      </div>

      {/* Mobile menu */}
      {menuOpen && (
        <div className="md:hidden bg-[#0a0a0f]/95 backdrop-blur-xl border-b border-[#9945FF]/20">
          <div className="px-4 py-4 space-y-3">
            {[
              { label: 'Features', href: '#solution' },
              { label: 'How it Works', href: '#how-it-works' },
              { label: 'Demo', href: '#demo' },
              { label: 'Research', href: '#research' },
              { label: 'Team', href: '#team' },
            ].map((link) => (
              <a
                key={link.label}
                href={link.href}
                onClick={() => setMenuOpen(false)}
                className="block text-slate-300 hover:text-white py-2 text-sm font-medium transition-colors"
              >
                {link.label}
              </a>
            ))}
            <a
              href="/demo/"
              className="block w-full text-center px-5 py-2.5 rounded-lg bg-gradient-to-r from-[#9945FF] to-[#7c3aed] text-white text-sm font-semibold mt-2"
            >
              Try Demo →
            </a>
          </div>
        </div>
      )}
    </nav>
  )
}

// ─── Hero ─────────────────────────────────────────────────────────────────────

function Hero() {
  return (
    <section className="relative min-h-screen flex flex-col items-center justify-center px-4 pt-16 overflow-hidden">
      {/* Background glow */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-[-20%] left-1/2 -translate-x-1/2 w-[800px] h-[600px] bg-[radial-gradient(ellipse_at_center,rgba(153,69,255,0.2)_0%,transparent_70%)]" />
        <div className="absolute top-[20%] left-[10%] w-[400px] h-[400px] bg-[radial-gradient(ellipse_at_center,rgba(153,69,255,0.08)_0%,transparent_70%)] blur-3xl" />
        <div className="absolute top-[30%] right-[5%] w-[300px] h-[300px] bg-[radial-gradient(ellipse_at_center,rgba(20,241,149,0.06)_0%,transparent_70%)] blur-3xl" />
        {/* Grid */}
        <div
          className="absolute inset-0 opacity-[0.03]"
          style={{
            backgroundImage: `linear-gradient(rgba(153,69,255,1) 1px, transparent 1px), linear-gradient(90deg, rgba(153,69,255,1) 1px, transparent 1px)`,
            backgroundSize: '60px 60px',
          }}
        />
      </div>

      <div className="relative z-10 max-w-5xl mx-auto text-center">
        {/* Hackathon tag */}
        <div className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-[#9945FF]/40 bg-[#9945FF]/10 text-[#c084fc] text-xs font-semibold tracking-widest uppercase mb-8 backdrop-blur-sm">
          <span className="w-1.5 h-1.5 rounded-full bg-[#14F195] animate-pulse" />
          Solana Frontier Hackathon 2026
        </div>

        {/* H1 */}
        <h1 className="text-5xl sm:text-6xl lg:text-7xl font-black tracking-tight leading-[1.05] mb-6">
          <span className="text-white">Zero-Movement PQC</span>
          <br />
          <span className="gradient-text">Account Abstraction</span>
          <br />
          <span className="text-white">for Solana</span>
        </h1>

        {/* Subtext */}
        <p className="max-w-2xl mx-auto text-lg sm:text-xl text-slate-400 leading-relaxed mb-10">
          Upgrade your Solana wallet to post-quantum security, key rotation, and cross-chain
          identity — <span className="text-slate-200 font-medium">without moving a single asset.</span>
        </p>

        {/* CTAs */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
          <a
            href="/demo/"
            className="group flex items-center gap-2 px-8 py-3.5 rounded-xl bg-gradient-to-r from-[#9945FF] to-[#7c3aed] text-white font-bold text-base hover:shadow-[0_0_30px_rgba(153,69,255,0.5)] transition-all duration-200 hover:-translate-y-0.5"
          >
            Try Demo
            <span className="group-hover:translate-x-1 transition-transform duration-200">→</span>
          </a>
          <a
            href="https://github.com/acechain-io/solana-pqc-account"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 px-8 py-3.5 rounded-xl border border-[#9945FF]/40 text-slate-200 font-bold text-base hover:border-[#9945FF] hover:bg-[#9945FF]/10 transition-all duration-200 hover:-translate-y-0.5"
          >
            <IconGithub className="w-5 h-5" />
            View on GitHub
          </a>
          <a
            href="/whitepaper.pdf"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 px-8 py-3.5 rounded-xl border border-[#22d3ee]/40 text-slate-200 font-bold text-base hover:border-[#22d3ee] hover:bg-[#22d3ee]/10 transition-all duration-200 hover:-translate-y-0.5"
          >
            <IconDocument className="w-5 h-5" />
            Whitepaper
          </a>
        </div>

        {/* Stats row */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 max-w-3xl mx-auto">
          {[
            { value: '128 bytes', label: 'proof size' },
            { value: '52ms', label: 'proof gen' },
            { value: '$0.00', label: 'migration fee' },
            { value: '7 papers', label: 'research papers' },
          ].map((stat) => (
            <div
              key={stat.label}
              className="px-4 py-4 rounded-xl border border-[#9945FF]/15 bg-[#9945FF]/5 backdrop-blur-sm hover:border-[#9945FF]/30 transition-colors"
            >
              <div className="text-2xl font-black gradient-text mb-1">{stat.value}</div>
              <div className="text-xs text-slate-500 font-medium uppercase tracking-wide">{stat.label}</div>
            </div>
          ))}
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-slate-600">
        <span className="text-xs uppercase tracking-widest">Scroll</span>
        <div className="w-px h-8 bg-gradient-to-b from-[#9945FF]/50 to-transparent animate-pulse" />
      </div>
    </section>
  )
}

// ─── Problem section ──────────────────────────────────────────────────────────

function ProblemSection() {
  const problems = [
    {
      icon: <IconKey className="w-6 h-6" />,
      title: 'Key Upgrade = Asset Migration',
      body: 'Changing to a safer key means creating a new address and moving everything: $50–200+ in fees, hours of work. There is no way to upgrade your key in place.',
      accent: '#FF6B6B',
      bg: 'rgba(255,107,107,0.08)',
      border: 'rgba(255,107,107,0.2)',
    },
    {
      icon: <IconShield className="w-6 h-6" />,
      title: 'Post-Quantum Threat',
      body: "Solana uses Ed25519. Quantum computers can break it with Shor's algorithm. No PQC migration path exists that preserves your address.",
      accent: '#FFB347',
      bg: 'rgba(255,179,71,0.08)',
      border: 'rgba(255,179,71,0.2)',
    },
    {
      icon: <IconGlobe className="w-6 h-6" />,
      title: 'Cross-Chain Identity Split',
      body: 'Users hold assets on Bitcoin, Ethereum, Solana under separate keys. Bridges to prove ownership have led to $2B+ in exploits.',
      accent: '#FF6B9D',
      bg: 'rgba(255,107,157,0.08)',
      border: 'rgba(255,107,157,0.2)',
    },
  ]

  return (
    <section id="problem" className="py-24 px-4 relative">
      <div className="max-w-6xl mx-auto">
        {/* Section label */}
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-red-500/30 bg-red-500/10 text-red-400 text-xs font-semibold tracking-widest uppercase mb-4">
            The Problem
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            Solana Security Has a <span className="text-red-400">Fundamental Gap</span>
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            Three unsolved problems that put every Solana wallet at risk today and tomorrow.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-6">
          {problems.map((p) => (
            <div
              key={p.title}
              className="rounded-2xl p-6 border transition-all duration-300 hover:-translate-y-1 hover:shadow-xl"
              style={{
                background: p.bg,
                borderColor: p.border,
              }}
            >
              <div
                className="w-12 h-12 rounded-xl flex items-center justify-center mb-5"
                style={{ background: p.bg, border: `1px solid ${p.border}`, color: p.accent }}
              >
                {p.icon}
              </div>
              <h3 className="text-white font-bold text-lg mb-3">{p.title}</h3>
              <p className="text-slate-400 text-sm leading-relaxed">{p.body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Solution section ─────────────────────────────────────────────────────────

function SolutionSection() {
  const layers = [
    {
      num: '01',
      tag: 'Layer 1',
      title: 'SA-Migration',
      subtitle: 'Zero-Movement Key Upgrade',
      color: '#9945FF',
      items: [
        'Import existing Phantom/Solflare mnemonic',
        'Same address preserved — no migration needed',
        'AES-256-GCM-SIV encrypted storage',
        'Zero on-chain transactions during upgrade',
      ],
    },
    {
      num: '02',
      tag: 'Layer 2',
      title: 'SolAA Smart Account',
      subtitle: 'PDA-Based Account Abstraction',
      color: '#7c3aed',
      items: [
        'PDA-based smart account on Solana',
        'ZK-proof authorized transactions (Groth16, ~170K CU)',
        'Key rotation without moving assets',
        'Social recovery with timelock protection',
      ],
    },
    {
      num: '03',
      tag: 'Layer 3',
      title: 'ZK-Ownership',
      subtitle: 'Cross-Chain Identity Proofs',
      color: '#14F195',
      items: [
        'Cross-chain ownership proofs',
        'Prove you own an Ethereum address from Solana',
        'No bridge required — ZK-proof only',
        '~280K CU verification on-chain',
      ],
    },
  ]

  return (
    <section id="solution" className="py-24 px-4 relative overflow-hidden">
      {/* BG */}
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-[radial-gradient(ellipse_at_center,rgba(153,69,255,0.07)_0%,transparent_70%)]" />
      </div>

      <div className="relative max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#9945FF]/30 bg-[#9945FF]/10 text-[#c084fc] text-xs font-semibold tracking-widest uppercase mb-4">
            The Solution
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            Three Layers, <span className="gradient-text">One Identity</span>
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            A composable stack that preserves your existing address while adding post-quantum
            security, smart account features, and cross-chain identity.
          </p>
        </div>

        <div className="grid lg:grid-cols-3 gap-6">
          {layers.map((layer) => (
            <div
              key={layer.title}
              className="rounded-2xl p-6 border border-[#9945FF]/15 bg-gradient-to-b from-[#0f0f1a] to-[#0a0a12] hover:border-[#9945FF]/30 hover:-translate-y-1 transition-all duration-300 group"
            >
              <div className="flex items-start justify-between mb-5">
                <div>
                  <span
                    className="text-xs font-bold tracking-widest uppercase px-2.5 py-1 rounded-full"
                    style={{
                      color: layer.color,
                      background: `${layer.color}15`,
                      border: `1px solid ${layer.color}30`,
                    }}
                  >
                    {layer.tag}
                  </span>
                </div>
                <span className="text-4xl font-black opacity-10 group-hover:opacity-20 transition-opacity" style={{ color: layer.color }}>
                  {layer.num}
                </span>
              </div>

              <h3 className="text-xl font-black text-white mb-1">{layer.title}</h3>
              <p className="text-sm font-medium mb-5" style={{ color: layer.color }}>
                {layer.subtitle}
              </p>

              <ul className="space-y-2.5">
                {layer.items.map((item) => (
                  <li key={item} className="flex items-start gap-2.5 text-sm text-slate-400">
                    <span
                      className="w-4 h-4 rounded-full flex-shrink-0 flex items-center justify-center mt-0.5"
                      style={{ background: `${layer.color}20`, border: `1px solid ${layer.color}40` }}
                    >
                      <svg viewBox="0 0 10 10" className="w-2.5 h-2.5" fill="none" stroke={layer.color} strokeWidth="2">
                        <path d="M2 5l2.5 2.5 4-4" strokeLinecap="round" strokeLinejoin="round" />
                      </svg>
                    </span>
                    <span>{item}</span>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Technical Specs / Comparison ────────────────────────────────────────────

function TechSpecsSection() {
  const rows = [
    {
      feature: 'Asset movement required',
      ace: { value: 'No', good: true },
      erc: { value: 'Yes', good: false },
      squads: { value: 'Yes', good: false },
    },
    {
      feature: 'Original address preserved',
      ace: { value: 'Yes', good: true },
      erc: { value: 'No', good: false },
      squads: { value: 'No', good: false },
    },
    {
      feature: 'Migration fee',
      ace: { value: '$0.00', good: true },
      erc: { value: 'High', good: false },
      squads: { value: 'Medium', good: false },
    },
    {
      feature: 'Post-quantum ready',
      ace: { value: 'Yes (ML-DSA-44)', good: true },
      erc: { value: 'No', good: false },
      squads: { value: 'No', good: false },
    },
    {
      feature: 'Key rotation',
      ace: { value: 'Yes (ZK proof)', good: true },
      erc: { value: 'Yes (new addr)', good: false },
      squads: { value: 'Limited', good: false },
    },
    {
      feature: 'Cross-chain ownership',
      ace: { value: 'Yes', good: true },
      erc: { value: 'No', good: false },
      squads: { value: 'No', good: false },
    },
  ]

  return (
    <section id="specs" className="py-24 px-4 bg-[#080810]">
      <div className="max-w-5xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#14F195]/30 bg-[#14F195]/10 text-[#14F195] text-xs font-semibold tracking-widest uppercase mb-4">
            Technical Specs
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            Built <span className="gradient-text">Different</span>
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            How SolAA compares to existing account abstraction approaches.
          </p>
        </div>

        <div className="rounded-2xl overflow-hidden border border-[#9945FF]/20">
          <div className="overflow-x-auto">
            <table className="w-full ace-table">
              <thead>
                <tr>
                  <th className="text-left">Feature</th>
                  <th className="text-center">
                    <span className="gradient-text">SolAA</span>
                  </th>
                  <th className="text-center text-slate-400">ERC-4337 Style</th>
                  <th className="text-center text-slate-400">Squads Multisig</th>
                </tr>
              </thead>
              <tbody>
                {rows.map((row) => (
                  <tr key={row.feature}>
                    <td className="text-slate-300 font-medium">{row.feature}</td>
                    <td className="text-center">
                      <span
                        className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold ${
                          row.ace.good
                            ? 'bg-[#14F195]/10 text-[#14F195] border border-[#14F195]/20'
                            : 'bg-red-500/10 text-red-400 border border-red-500/20'
                        }`}
                      >
                        {row.ace.good ? (
                          <IconCheck className="w-3 h-3" />
                        ) : (
                          <IconX className="w-3 h-3" />
                        )}
                        {row.ace.value}
                      </span>
                    </td>
                    <td className="text-center">
                      <span
                        className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold ${
                          row.erc.good
                            ? 'bg-[#14F195]/10 text-[#14F195] border border-[#14F195]/20'
                            : 'bg-slate-500/10 text-slate-500 border border-slate-500/20'
                        }`}
                      >
                        {row.erc.good ? (
                          <IconCheck className="w-3 h-3" />
                        ) : (
                          <IconX className="w-3 h-3" />
                        )}
                        {row.erc.value}
                      </span>
                    </td>
                    <td className="text-center">
                      <span
                        className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold ${
                          row.squads.good
                            ? 'bg-[#14F195]/10 text-[#14F195] border border-[#14F195]/20'
                            : 'bg-slate-500/10 text-slate-500 border border-slate-500/20'
                        }`}
                      >
                        {row.squads.good ? (
                          <IconCheck className="w-3 h-3" />
                        ) : (
                          <IconX className="w-3 h-3" />
                        )}
                        {row.squads.value}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── How it Works ─────────────────────────────────────────────────────────────

function HowItWorksSection() {
  const steps = [
    {
      title: 'Import your existing seed phrase',
      detail: 'Import your existing Phantom or Solflare mnemonic. Your identity is derived deterministically — no new keys generated yet.',
      code: 'solaa import --mnemonic "your twelve word phrase..."',
    },
    {
      title: 'Address preserved — zero transactions',
      detail: 'Your original Solana address is preserved mathematically. No on-chain transaction is needed to "register" — it just works.',
      code: 'Original address: 7xKXt...9mPq ✓ preserved',
    },
    {
      title: 'Create SolAA Smart Account (PDA)',
      detail: 'Deploy a PDA-based smart account. Fund it and use it like a normal wallet — but with programmable authorization rules.',
      code: 'solaa create-account --fund 0.1 SOL',
    },
    {
      title: 'Execute via 128-byte ZK proof',
      detail: 'Every transaction is authorized by a compact Groth16 ZK proof. 128 bytes, generated client-side in ~52ms. ~170K compute units on-chain.',
      code: 'Proof: [0x2a, 0x9f, ...] 128 bytes · 52ms · 170K CU',
    },
    {
      title: 'Rotate to ML-DSA-44 (post-quantum)',
      detail: 'When ready, rotate your key from Ed25519 to ML-DSA-44 in a single transaction. Your address stays the same. Quantum-safe from day one.',
      code: 'solaa rotate --algorithm ML-DSA-44 --preserve-address',
    },
  ]

  return (
    <section id="how-it-works" className="py-24 px-4 relative overflow-hidden">
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute right-0 top-1/2 -translate-y-1/2 w-96 h-96 bg-[radial-gradient(ellipse_at_right,rgba(20,241,149,0.05)_0%,transparent_70%)]" />
      </div>

      <div className="relative max-w-4xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#9945FF]/30 bg-[#9945FF]/10 text-[#c084fc] text-xs font-semibold tracking-widest uppercase mb-4">
            How it Works
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            From Legacy Wallet to{' '}
            <span className="gradient-text">Quantum-Safe</span>
            {' '}in 5 Steps
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            The entire migration takes under 2 minutes and costs nothing except a small PDA rent deposit.
          </p>
        </div>

        <div className="space-y-6">
          {steps.map((step, idx) => (
            <div key={step.title} className="flex gap-6 group">
              {/* Step number */}
              <div className="flex-shrink-0 flex flex-col items-center">
                <div className="w-10 h-10 rounded-full bg-gradient-to-br from-[#9945FF] to-[#7c3aed] flex items-center justify-center text-white font-black text-sm shadow-[0_0_15px_rgba(153,69,255,0.3)]">
                  {idx + 1}
                </div>
                {idx < steps.length - 1 && (
                  <div className="w-px flex-1 mt-2 bg-gradient-to-b from-[#9945FF]/30 to-transparent min-h-[2rem]" />
                )}
              </div>

              {/* Content */}
              <div className="flex-1 pb-8 last:pb-0">
                <h3 className="text-white font-bold text-lg mb-2 group-hover:text-[#c084fc] transition-colors">
                  {step.title}
                </h3>
                <p className="text-slate-400 text-sm leading-relaxed mb-3">{step.detail}</p>
                <div className="flex items-center gap-2 px-3 py-2 rounded-lg bg-black/40 border border-[#9945FF]/10 font-mono text-xs text-[#14F195] overflow-x-auto">
                  <span className="text-[#9945FF] flex-shrink-0">$</span>
                  <span className="whitespace-nowrap">{step.code}</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Research Foundation ──────────────────────────────────────────────────────

function ResearchSection() {
  const papers = [
    {
      id: 'ACE-GF',
      arxiv: 'arXiv:2511.20505',
      title: 'Deterministic identity derivation framework',
      url: 'https://arxiv.org/abs/2511.20505',
    },
    {
      id: 'ZK-ACE',
      arxiv: 'arXiv:2603.07974',
      title: 'ZK authorization for post-quantum cryptography',
      url: 'https://arxiv.org/abs/2603.07974',
    },
    {
      id: 'AR-ACE',
      arxiv: 'arXiv:2603.07982',
      title: 'Proof-off-path relay architecture',
      url: 'https://arxiv.org/abs/2603.07982',
    },

    {
      id: 'VA-DAR',
      arxiv: 'arXiv:2603.02690',
      title: 'Decentralized address recovery',
      url: 'https://arxiv.org/abs/2603.02690',
    },
    {
      id: 'CT-DAP',
      arxiv: 'arXiv:2603.07933',
      title: 'Destroyable authorization paths',
      url: 'https://arxiv.org/abs/2603.07933',
    },
    {
      id: 'ACE Runtime',
      arxiv: 'arXiv:2603.10242',
      title: 'Sub-second finality blockchain runtime',
      url: 'https://arxiv.org/abs/2603.10242',
    },
  ]

  return (
    <section id="research" className="py-24 px-4 bg-[#080810]">
      <div className="max-w-5xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#14F195]/30 bg-[#14F195]/10 text-[#14F195] text-xs font-semibold tracking-widest uppercase mb-4">
            Research Foundation
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            7 <span className="gradient-text">Research Papers</span>
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            SolAA is built on a solid research foundation. Every design decision
            is backed by published cryptographic research papers on arXiv.
          </p>
        </div>

        <div className="rounded-2xl overflow-hidden border border-[#9945FF]/20 mb-8">
          <div className="overflow-x-auto">
            <table className="w-full ace-table">
              <thead>
                <tr>
                  <th className="text-left">Paper</th>
                  <th className="text-left">Reference</th>
                  <th className="text-left">Contribution</th>
                  <th className="text-center">Link</th>
                </tr>
              </thead>
              <tbody>
                {papers.map((paper) => (
                  <tr key={paper.id}>
                    <td>
                      <span className="font-mono font-bold text-[#9945FF] text-sm">{paper.id}</span>
                    </td>
                    <td>
                      <span
                        className={`font-mono text-xs px-2 py-1 rounded ${
                          paper.arxiv === 'in submission'
                            ? 'text-amber-400 bg-amber-400/10 border border-amber-400/20'
                            : 'text-slate-400 bg-slate-800/50'
                        }`}
                      >
                        {paper.arxiv}
                      </span>
                    </td>
                    <td className="text-slate-300 text-sm">{paper.title}</td>
                    <td className="text-center">
                      {paper.url !== '#' ? (
                        <a
                          href={paper.url}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="inline-flex items-center gap-1 text-xs text-[#14F195] hover:text-[#14F195]/80 transition-colors font-medium"
                        >
                          <IconDocument className="w-3.5 h-3.5" />
                          PDF
                        </a>
                      ) : (
                        <span className="text-xs text-slate-600">Pending</span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

      </div>
    </section>
  )
}

// ─── Team section ─────────────────────────────────────────────────────────────

function TeamSection() {
  return (
    <section id="team" className="py-24 px-4 relative overflow-hidden">
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-[radial-gradient(ellipse_at_top,rgba(153,69,255,0.08)_0%,transparent_70%)]" />
      </div>

      <div className="relative max-w-4xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#9945FF]/30 bg-[#9945FF]/10 text-[#c084fc] text-xs font-semibold tracking-widest uppercase mb-4">
            Team
          </div>
          <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
            Built by <span className="gradient-text">Researchers</span>
          </h2>
          <p className="text-slate-400 max-w-xl mx-auto">
            Deep cryptographic expertise meets Solana protocol knowledge.
          </p>
        </div>

        <div className="flex justify-center">
          <div className="max-w-md w-full rounded-2xl p-8 border border-[#9945FF]/20 bg-gradient-to-b from-[#0f0f1a] to-[#0a0a12] hover:border-[#9945FF]/40 transition-all duration-300 hover:-translate-y-1 hover:shadow-[0_20px_60px_rgba(153,69,255,0.15)]">
            {/* Avatar */}
            <div className="flex items-center gap-5 mb-6">
              <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-[#9945FF] to-[#14F195] flex items-center justify-center text-white font-black text-2xl shadow-[0_0_20px_rgba(153,69,255,0.4)]">
                J
              </div>
              <div>
                <h3 className="text-white font-bold text-xl">Jason Wang</h3>
                <p className="text-[#9945FF] text-sm font-medium">Cryptography Researcher</p>
              </div>
            </div>

            <p className="text-slate-400 text-sm leading-relaxed mb-6">
              Author of the ACE-GF framework and all 7 related papers. Designed the
              identity-authorization separation architecture that enables zero-movement key
              rotation.
            </p>

            {/* Tags */}
            <div className="flex flex-wrap gap-2 mb-6">
              {[
                'ZK Proofs',
                'Post-Quantum Crypto',
                'Solana',
                'Account Abstraction',
                'ML-DSA-44',
                'Groth16',
              ].map((tag) => (
                <span
                  key={tag}
                  className="px-2.5 py-1 rounded-full bg-[#9945FF]/10 border border-[#9945FF]/20 text-[#c084fc] text-xs font-medium"
                >
                  {tag}
                </span>
              ))}
            </div>

            {/* Stats */}
            <div className="grid grid-cols-3 gap-4 pt-5 border-t border-[#9945FF]/15">
              {[
                { value: '7', label: 'Papers' },
                { value: '3', label: 'Protocols' },
              ].map((s) => (
                <div key={s.label} className="text-center">
                  <div className="text-xl font-black gradient-text">{s.value}</div>
                  <div className="text-slate-500 text-xs mt-0.5">{s.label}</div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Why SolAA ────────────────────────────────────────────────────────────────

function WhySolAASection() {
  const pillars = [
    {
      icon: IconShield,
      color: '#9945FF',
      bg: 'rgba(153,69,255,0.1)',
      border: 'rgba(153,69,255,0.25)',
      tag: 'Regulatory Tailwind',
      title: 'Quantum security is no longer optional',
      body: 'NIST finalized post-quantum cryptography standards in 2024. Regulators are already requiring financial institutions to upgrade. SolAA lets every Solana user and project become quantum-safe today — with zero asset movement, zero downtime, and zero migration cost.',
      stat: '$200B+',
      statLabel: 'in Solana assets addressable on day one',
    },
    {
      icon: IconKey,
      color: '#14F195',
      bg: 'rgba(20,241,149,0.08)',
      border: 'rgba(20,241,149,0.2)',
      tag: 'Unfair Advantage',
      title: 'Account abstraction without the migration tax',
      body: 'Every existing AA solution on EVM requires users to create a new wallet and move their assets. SolAA is different: users upgrade in place. Their address, history, and reputation stay intact. The addressable market is every current Solana user — not just the ones willing to start over.',
      stat: '0 txns',
      statLabel: 'required to fully upgrade security',
    },
    {
      icon: IconGlobe,
      color: '#22d3ee',
      bg: 'rgba(34,211,238,0.08)',
      border: 'rgba(34,211,238,0.2)',
      tag: 'The Missing Primitive',
      title: 'Cross-chain identity without bridges',
      body: "Bridges have been hacked for $2.5B+ since 2021. SolAA introduces a better model: instead of moving assets across chains, users prove ownership across chains with a single ZK proof. One Solana wallet becomes a universal identity for Ethereum, Tron, and any future chain.",
      stat: '$2.5B+',
      statLabel: 'lost to bridge hacks — SolAA eliminates the need',
    },
  ]

  return (
    <section id="why-solaa" className="py-24 px-4">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-5 py-2 rounded-full border border-[#9945FF]/60 bg-[#9945FF]/20 text-[#c084fc] text-sm font-bold uppercase tracking-widest mb-6 shadow-[0_0_20px_rgba(153,69,255,0.3)]">
            Why SolAA
          </div>
          <h2 className="text-4xl font-bold mb-4">
            Three reasons this <span className="gradient-text">matters now</span>
          </h2>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto">
            SolAA is not a research project. It is production infrastructure solving three problems that get more urgent every quarter.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          {pillars.map((p) => (
            <div
              key={p.title}
              className="rounded-2xl p-6 flex flex-col gap-4"
              style={{ background: p.bg, border: `1px solid ${p.border}` }}
            >
              <div className="flex items-center justify-between">
                <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: p.bg, border: `1px solid ${p.border}`, color: p.color }}>
                  <p.icon className="w-5 h-5" />
                </div>
                <span className="text-xs font-semibold px-2.5 py-1 rounded-full" style={{ color: p.color, background: p.bg, border: `1px solid ${p.border}` }}>
                  {p.tag}
                </span>
              </div>
              <div>
                <h3 className="text-white font-semibold text-lg mb-2 leading-snug">{p.title}</h3>
                <p className="text-slate-400 text-sm leading-relaxed">{p.body}</p>
              </div>
              <div className="mt-auto pt-4 border-t" style={{ borderColor: p.border }}>
                <div className="text-2xl font-bold" style={{ color: p.color }}>{p.stat}</div>
                <div className="text-slate-500 text-xs mt-0.5">{p.statLabel}</div>
              </div>
            </div>
          ))}
        </div>

        <div className="flex justify-center">
          <a
            href="/whitepaper.pdf"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-6 py-3 rounded-xl font-semibold text-sm transition-all"
            style={{ background: 'rgba(153,69,255,0.15)', border: '1px solid rgba(153,69,255,0.4)', color: '#c084fc' }}
          >
            <IconDocument className="w-4 h-4" />
            Read the Whitepaper
          </a>
        </div>
      </div>
    </section>
  )
}

// ─── CTA Banner ───────────────────────────────────────────────────────────────

function CTASection() {
  return (
    <section className="py-20 px-4">
      <div className="max-w-4xl mx-auto">
        <div className="relative rounded-3xl overflow-hidden border border-[#9945FF]/30 p-12 text-center">
          {/* Background gradient */}
          <div className="absolute inset-0 bg-gradient-to-br from-[#9945FF]/20 via-[#7c3aed]/10 to-[#14F195]/10" />
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,rgba(153,69,255,0.15)_0%,transparent_70%)]" />

          <div className="relative z-10">
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[#14F195]/40 bg-[#14F195]/10 text-[#14F195] text-xs font-semibold tracking-widest uppercase mb-6">
              <span className="w-1.5 h-1.5 rounded-full bg-[#14F195] animate-pulse" />
              Live on Solana Devnet
            </div>

            <h2 className="text-3xl sm:text-4xl font-black text-white mb-4">
              Ready to Go{' '}
              <span className="gradient-text">Post-Quantum?</span>
            </h2>
            <p className="text-slate-400 text-lg mb-8 max-w-xl mx-auto">
              Try the interactive demo or explore the open-source codebase. $0 migration fee.
              Your address stays the same.
            </p>

            <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
              <a
                href="/demo/"
                className="group flex items-center gap-2 px-8 py-3.5 rounded-xl bg-gradient-to-r from-[#9945FF] to-[#7c3aed] text-white font-bold text-base hover:shadow-[0_0_30px_rgba(153,69,255,0.5)] transition-all duration-200 hover:-translate-y-0.5"
              >
                Try Demo
                <span className="group-hover:translate-x-1 transition-transform">→</span>
              </a>
              <a
                href="https://github.com/acechain-io/solana-pqc-account"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 px-8 py-3.5 rounded-xl border border-white/20 text-white font-bold text-base hover:bg-white/5 transition-all duration-200 hover:-translate-y-0.5"
              >
                <IconGithub className="w-5 h-5" />
                GitHub Repository
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Footer ───────────────────────────────────────────────────────────────────

function Footer() {
  const papers = [
    { id: 'ACE-GF', url: 'https://arxiv.org/abs/2511.20505' },
    { id: 'ZK-ACE', url: 'https://arxiv.org/abs/2603.07974' },
    { id: 'AR-ACE', url: 'https://arxiv.org/abs/2603.07982' },
    { id: 'VA-DAR', url: 'https://arxiv.org/abs/2603.02690' },
    { id: 'CT-DAP', url: 'https://arxiv.org/abs/2603.07933' },
    { id: 'ACE Runtime', url: 'https://arxiv.org/abs/2603.10242' },
  ]

  return (
    <footer className="border-t border-[#9945FF]/10 bg-[#080810]">
      <div className="max-w-6xl mx-auto px-4 py-12">
        <div className="grid md:grid-cols-3 gap-8 mb-10">
          {/* Brand */}
          <div>
            <div className="flex items-center gap-2.5 mb-3">
              <img src="/solaa-logo.svg" alt="SolAA" className="w-7 h-7" />
              <span className="font-bold text-white">SolAA</span>
            </div>
            <p className="text-slate-500 text-sm leading-relaxed">
              Post-quantum account abstraction for Solana. Zero-movement key rotation, ZK proof
              authorization, cross-chain identity.
            </p>
          </div>

          {/* Links */}
          <div>
            <h4 className="text-white font-semibold text-sm mb-4 uppercase tracking-wider">Research Papers</h4>
            <div className="grid grid-cols-2 gap-1.5">
              {papers.map((p) => (
                <a
                  key={p.id}
                  href={p.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-slate-500 hover:text-[#14F195] text-xs font-mono transition-colors"
                >
                  {p.id}
                </a>
              ))}
            </div>
          </div>

          {/* Patent & GitHub */}
          <div>
            <h4 className="text-white font-semibold text-sm mb-4 uppercase tracking-wider">Links</h4>
            <div className="space-y-2">
              <a
                href="https://github.com/acechain-io/solana-pqc-account"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 text-slate-500 hover:text-white text-sm transition-colors"
              >
                <IconGithub className="w-4 h-4" />
                GitHub Repository
              </a>
              <a
                href="/whitepaper.pdf"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 text-slate-500 hover:text-white text-sm transition-colors"
              >
                <IconDocument className="w-4 h-4" />
                Whitepaper (PDF)
              </a>
            </div>
          </div>
        </div>

        <div className="pt-8 border-t border-[#9945FF]/10 flex flex-col sm:flex-row items-center justify-between gap-3">
          <p className="text-slate-600 text-sm text-center sm:text-left">
            SolAA · Solana Frontier Hackathon 2026 · Built by{' '}
            <span className="text-slate-400">Jason Wang</span>
          </p>
          <div className="flex items-center gap-1.5">
            <span className="w-1.5 h-1.5 rounded-full bg-[#14F195] animate-pulse" />
            <span className="text-slate-600 text-xs">Solana Devnet</span>
          </div>
        </div>
      </div>
    </footer>
  )
}

// ─── Page ─────────────────────────────────────────────────────────────────────

export default function Page() {
  return (
    <main>
      <Navbar />
      <Hero />
      <ProblemSection />
      <WhySolAASection />
      <SolutionSection />
      <TechSpecsSection />
      <HowItWorksSection />
      <LiveDemoSection />
      <ResearchSection />
      <TeamSection />
      <CTASection />
      <Footer />
    </main>
  )
}
