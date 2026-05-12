import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AceAccountKit } from "../target/types/ace_account_kit";
import { expect } from "chai";
import {
  Keypair,
  SystemProgram,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { createHash, randomBytes } from "crypto";
import BN from "bn.js";

describe("solaa", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AceAccountKit as Program<AceAccountKit>;
  const payer = provider.wallet as anchor.Wallet;

  // ─── Test fixtures ──────────────────────────────────────────────

  /** Generate a deterministic id_com from test inputs. */
  function computeIdCom(
    rev: Buffer,
    salt: Buffer,
    domain: number
  ): Buffer {
    const domainBuf = Buffer.alloc(8);
    domainBuf.writeBigUInt64BE(BigInt(domain));
    return createHash("sha256")
      .update(rev)
      .update(salt)
      .update(domainBuf)
      .digest();
  }

  /** Create a dummy ZK proof (all zeros — will pass with placeholder VK). */
  function dummyProof() {
    return {
      a: Array.from(Buffer.alloc(64)),
      b: Array.from(Buffer.alloc(128)),
      c: Array.from(Buffer.alloc(64)),
    };
  }

  /** Create dummy public inputs (5 × 32 bytes). */
  function dummyPublicInputs(
    idCom: Buffer,
    txHash: Buffer,
    domain: number,
    target: Buffer,
    rpCom: Buffer
  ): Buffer {
    const domainField = Buffer.alloc(32);
    domainField.writeBigUInt64BE(BigInt(domain), 24);

    return Buffer.concat([idCom, txHash, domainField, target, rpCom]);
  }

  /** Derive PDA for a smart account. */
  function derivePda(idCom: Buffer): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("solaa"), idCom],
      program.programId
    );
  }

  // ─── Test data ──────────────────────────────────────────────────

  const testRev = randomBytes(32);
  const testSalt = randomBytes(32);
  const testDomain = 2; // devnet
  const testIdCom = computeIdCom(testRev, testSalt, testDomain);
  const guardian = Keypair.generate();

  // ═══════════════════════════════════════════════════════════════
  //  1. Initialize
  // ═══════════════════════════════════════════════════════════════

  describe("initialize", () => {
    it("creates a SolAA Smart Account with correct state", async () => {
      const [pda] = derivePda(testIdCom);

      await program.methods
        .initialize({
          idCom: Array.from(testIdCom),
          domain: new BN(testDomain),
          guardian: guardian.publicKey,
          recoveryDelay: new BN(100), // 100 slots for testing
        })
        .accounts({
          smartAccount: pda,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      // Fetch and verify state
      const account = await program.account.aceSmartAccount.fetch(pda);

      expect(Buffer.from(account.idCom)).to.deep.equal(testIdCom);
      expect(account.nonce.toNumber()).to.equal(0);
      expect(account.domain.toNumber()).to.equal(testDomain);
      expect(account.guardian.toBase58()).to.equal(
        guardian.publicKey.toBase58()
      );
      expect(account.recoveryDelay.toNumber()).to.equal(100);
      expect(account.pendingRecovery).to.be.null;
    });

    it("rejects duplicate initialization (same id_com)", async () => {
      const [pda] = derivePda(testIdCom);

      try {
        await program.methods
          .initialize({
            idCom: Array.from(testIdCom),
            domain: new BN(testDomain),
            guardian: null,
            recoveryDelay: null,
          })
          .accounts({
            smartAccount: pda,
            payer: payer.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();

        expect.fail("Should have thrown");
      } catch (err: any) {
        // Account already exists — this is expected
        expect(err.toString()).to.include("already in use");
      }
    });

    it("creates account without guardian", async () => {
      const noGuardianRev = randomBytes(32);
      const noGuardianSalt = randomBytes(32);
      const noGuardianIdCom = computeIdCom(noGuardianRev, noGuardianSalt, testDomain);
      const [pda] = derivePda(noGuardianIdCom);

      await program.methods
        .initialize({
          idCom: Array.from(noGuardianIdCom),
          domain: new BN(testDomain),
          guardian: null,
          recoveryDelay: null,
        })
        .accounts({
          smartAccount: pda,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const account = await program.account.aceSmartAccount.fetch(pda);
      expect(account.guardian).to.be.null;
      // Default recovery delay should be set
      expect(account.recoveryDelay.toNumber()).to.be.greaterThan(0);
    });
  });

  // ═══════════════════════════════════════════════════════════════
  //  2. Execute (ZK-authorized transaction)
  // ═══════════════════════════════════════════════════════════════

  describe("execute", () => {
    it("accepts a valid proof and bumps nonce", async () => {
      const [pda] = derivePda(testIdCom);

      const payload = Buffer.from("transfer 1 SOL to Alice");
      const txHash = createHash("sha256").update(payload).digest();
      const target = randomBytes(32);
      const rpCom = randomBytes(32);

      const pubInputs = dummyPublicInputs(
        testIdCom,
        txHash,
        testDomain,
        target,
        rpCom
      );

      await program.methods
        .execute({
          payload: Array.from(payload),
          proof: dummyProof(),
          publicInputsBytes: Array.from(pubInputs),
        })
        .accounts({
          smartAccount: pda,
          submitter: payer.publicKey,
        })
        .rpc();

      const account = await program.account.aceSmartAccount.fetch(pda);
      expect(account.nonce.toNumber()).to.equal(1);
    });

    it("rejects proof with wrong id_com", async () => {
      const [pda] = derivePda(testIdCom);

      const wrongIdCom = randomBytes(32);
      const pubInputs = dummyPublicInputs(
        wrongIdCom,
        randomBytes(32),
        testDomain,
        randomBytes(32),
        randomBytes(32)
      );

      try {
        await program.methods
          .execute({
            payload: Array.from(Buffer.from("bad")),
            proof: dummyProof(),
            publicInputsBytes: Array.from(pubInputs),
          })
          .accounts({
            smartAccount: pda,
            submitter: payer.publicKey,
          })
          .rpc();

        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.toString()).to.include("IdComMismatch");
      }
    });

    it("rejects proof with wrong domain", async () => {
      const [pda] = derivePda(testIdCom);

      const pubInputs = dummyPublicInputs(
        testIdCom,
        randomBytes(32),
        99, // wrong domain
        randomBytes(32),
        randomBytes(32)
      );

      try {
        await program.methods
          .execute({
            payload: Array.from(Buffer.from("bad")),
            proof: dummyProof(),
            publicInputsBytes: Array.from(pubInputs),
          })
          .accounts({
            smartAccount: pda,
            submitter: payer.publicKey,
          })
          .rpc();

        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.toString()).to.include("DomainMismatch");
      }
    });
  });

  // ═══════════════════════════════════════════════════════════════
  //  3. Rotate Key
  // ═══════════════════════════════════════════════════════════════

  describe("rotate_key", () => {
    it("rotates id_com while preserving PDA address", async () => {
      const rev = randomBytes(32);
      const salt = randomBytes(32);
      const idCom = computeIdCom(rev, salt, testDomain);
      const [pda] = derivePda(idCom);

      await program.methods
        .initialize({
          idCom: Array.from(idCom),
          domain: new BN(testDomain),
          guardian: null,
          recoveryDelay: null,
        })
        .accounts({
          smartAccount: pda,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      // Record PDA address before rotation
      const pdaBefore = pda.toBase58();

      // Rotate key
      const newIdCom = randomBytes(32);
      const pubInputs = dummyPublicInputs(
        idCom,
        randomBytes(32),
        testDomain,
        randomBytes(32),
        randomBytes(32)
      );

      await program.methods
        .rotateKey({
          newIdCom: Array.from(newIdCom),
          proof: dummyProof(),
          publicInputsBytes: Array.from(pubInputs),
        })
        .accounts({
          smartAccount: pda,
          submitter: payer.publicKey,
        })
        .rpc();

      // PDA address unchanged
      const account = await program.account.aceSmartAccount.fetch(pda);
      expect(pda.toBase58()).to.equal(pdaBefore);

      // id_com updated
      expect(Buffer.from(account.idCom)).to.deep.equal(newIdCom);
      expect(account.nonce.toNumber()).to.equal(1);
    });
  });

  // ═══════════════════════════════════════════════════════════════
  //  4. Social Recovery
  // ═══════════════════════════════════════════════════════════════

  describe("social_recovery", () => {
    let recoveryIdCom: Buffer;
    let recoveryPda: PublicKey;

    before(async () => {
      const rev = randomBytes(32);
      const salt = randomBytes(32);
      recoveryIdCom = computeIdCom(rev, salt, testDomain);
      [recoveryPda] = derivePda(recoveryIdCom);

      await program.methods
        .initialize({
          idCom: Array.from(recoveryIdCom),
          domain: new BN(testDomain),
          guardian: guardian.publicKey,
          recoveryDelay: new BN(1), // 1 slot for fast testing
        })
        .accounts({
          smartAccount: recoveryPda,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
    });

    it("guardian can initiate recovery", async () => {
      const newIdCom = randomBytes(32);

      // Fund guardian
      const sig = await provider.connection.requestAirdrop(
        guardian.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      await program.methods
        .initiateRecovery({
          newIdCom: Array.from(newIdCom),
        })
        .accounts({
          smartAccount: recoveryPda,
          guardian: guardian.publicKey,
        })
        .signers([guardian])
        .rpc();

      const account = await program.account.aceSmartAccount.fetch(recoveryPda);
      expect(account.pendingRecovery).to.not.be.null;
      expect(Buffer.from(account.pendingRecovery.newIdCom)).to.deep.equal(
        newIdCom
      );
    });

    it("non-guardian cannot initiate recovery", async () => {
      // Create a separate account for this test
      const rev2 = randomBytes(32);
      const salt2 = randomBytes(32);
      const idCom2 = computeIdCom(rev2, salt2, testDomain);
      const [pda2] = derivePda(idCom2);

      await program.methods
        .initialize({
          idCom: Array.from(idCom2),
          domain: new BN(testDomain),
          guardian: guardian.publicKey,
          recoveryDelay: new BN(1),
        })
        .accounts({
          smartAccount: pda2,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const fakeGuardian = Keypair.generate();
      const airdropSig = await provider.connection.requestAirdrop(
        fakeGuardian.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);

      try {
        await program.methods
          .initiateRecovery({
            newIdCom: Array.from(randomBytes(32)),
          })
          .accounts({
            smartAccount: pda2,
            guardian: fakeGuardian.publicKey,
          })
          .signers([fakeGuardian])
          .rpc();

        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.toString()).to.include("UnauthorizedGuardian");
      }
    });

    it("can finalize recovery after timelock", async () => {
      const account = await program.account.aceSmartAccount.fetch(recoveryPda);
      const newIdCom = Buffer.from(account.pendingRecovery.newIdCom);

      // Wait for slot to advance past timelock
      await new Promise((r) => setTimeout(r, 1500));

      await program.methods
        .finalizeRecovery()
        .accounts({
          smartAccount: recoveryPda,
          caller: payer.publicKey,
        })
        .rpc();

      const updated = await program.account.aceSmartAccount.fetch(recoveryPda);
      expect(Buffer.from(updated.idCom)).to.deep.equal(newIdCom);
      expect(updated.pendingRecovery).to.be.null;
    });
  });

  // ═══════════════════════════════════════════════════════════════
  //  5. ZK-Ownership (cross-chain proof)
  // ═══════════════════════════════════════════════════════════════

  describe("verify_ownership", () => {
    it("creates an on-chain ownership record", async () => {
      const foreignAddress = randomBytes(32);
      const foreignChain = 1; // Ethereum

      const foreignChainBytes = Buffer.alloc(8);
      foreignChainBytes.writeBigUInt64LE(BigInt(foreignChain));

      const [ownershipPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("solaa-own"),
          foreignAddress,
          foreignChainBytes,
        ],
        program.programId
      );

      const idCom = randomBytes(32);
      const solanaAddr = randomBytes(32);
      const chainField = Buffer.alloc(32);
      chainField.writeBigUInt64BE(BigInt(foreignChain), 24);

      const ownershipPubInputs = Buffer.concat([
        idCom,
        solanaAddr,
        foreignAddress,
        chainField,
      ]);

      await program.methods
        .verifyOwnership({
          foreignAddress: Array.from(foreignAddress),
          foreignChain: new BN(foreignChain),
          proof: dummyProof(),
          publicInputsBytes: Array.from(ownershipPubInputs),
        })
        .accounts({
          ownershipRecord: ownershipPda,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await program.account.ownershipRecord.fetch(ownershipPda);
      expect(Buffer.from(record.idCom)).to.deep.equal(idCom);
      expect(Buffer.from(record.foreignAddress)).to.deep.equal(foreignAddress);
      expect(record.foreignChain.toNumber()).to.equal(foreignChain);
    });
  });
});
