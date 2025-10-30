import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { MintTokenPats } from "../target/types/mint_token_pats";

describe("mint-token-pats", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.mintTokenPats as Program<MintTokenPats>;
  const wallet = provider.wallet;
  const connection = provider.connection;


  const mintKeypair = anchor.web3.Keypair.generate();

  it("Initializes PATS mint", async () => {
    // Get rent-exemption balance
    const space = 82;
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(space);

    // Create the mint account
    const txSig = await program.methods
      .initialize() 
      .accounts({
        creator: provider.wallet.publicKey,
        mint: mintKeypair.publicKey,
        token_program: new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"), // Token-2022 ID
        systemProgram: SystemProgram.programId,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("✅ InitializeMint Tx:", txSig);
    console.log(`🔗 Explorer: https://explorer.solana.com/tx/${txSig}?cluster=devnet`);

  });
    it("💰 Mints 1,000 PATS to your wallet ATA", async () => {
    // 3️⃣ Create or get ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintKeypair.publicKey,
      wallet.publicKey,
      true,
      "confirmed",
      {},
      TOKEN_2022_PROGRAM_ID
    );

    // 4️⃣ Mint tokens
    const mintTx = await mintTo(
      connection,
      wallet.payer,
      mintKeypair.publicKey,
      ata.address,
      wallet.payer,
      1_000_000_000, // 1,000 tokens if 6 decimals
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    console.log("\n💸 Minted 1000 PATS tokens!");
    console.log("MintTo Tx:", mintTx);
    console.log(`🔗 Explorer: https://explorer.solana.com/tx/${mintTx}?cluster=devnet`); 
  });
});
