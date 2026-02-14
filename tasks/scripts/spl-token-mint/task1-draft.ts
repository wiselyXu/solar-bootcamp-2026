/** Challenge: Mint an SPL Token
 *
 * In this challenge, you will create an SPL token!
 *
 * Goal:
 *   Mint an SPL token in a single transaction using Web3.js and the SPL Token library.
 *
 * Objectives:
 *   1. Create an SPL mint account.
 *   2. Initialize the mint with 6 decimals and your public key (feePayer) as the mint and freeze authorities.
 *   3. Create an associated token account for your public key (feePayer) to hold the minted tokens.
 *   4. Mint 21,000,000 tokens to your associated token account.
 *   5. Sign and send the transaction.
 */

import {
    Keypair,
    Connection,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
  } from "@solana/web3.js";
  
  import {
    createAssociatedTokenAccountInstruction,
    createInitializeMint2Instruction,
    createMintToInstruction,
    createMintToCheckedInstruction,
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
  
    ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountIdempotentInstruction
  } from "@solana/spl-token";
  
  import bs58 from "bs58";
  
  // Import our keypair from the wallet file
  const payer = Keypair.fromSecretKey(
    // ⚠️ INSECURE KEY. DO NOT USE OUTSIDE OF THIS CHALLENGE
    bs58.decode(process.env.SECRET)
  );
  
  console.log("feepay address",payer.publicKey)
  //Create a connection to the RPC endpoint
  const connection = new Connection(
    process.env.RPC_ENDPOINT,
    "confirmed"
  );
  
  // Entry point of your TypeScript code (we will call this)
  async function main() {
    try {
  
      const mint = Keypair.generate();
  
      const mintRent = await getMinimumBalanceForRentExemptMint(connection);
  const createAccountInstruction = SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: mint.publicKey,
          space: MINT_SIZE,
          lamports: mintRent,
          programId: TOKEN_PROGRAM_ID
      });
  
  const initializeMintInstruction = createInitializeMint2Instruction(
          mint.publicKey, // mint pubkey
          6, // decimals
          payer.publicKey, // mint authority
          null, // freeze authority
          TOKEN_PROGRAM_ID
      );
  
      var transaction = new Transaction().add(
          createAccountInstruction,
          initializeMintInstruction,
      );
  
    var signature = await sendAndConfirmTransaction(connection, transaction, [payer, mint]);
  
      console.log("Mint Address:", mint.publicKey.toBase58());
      console.log("Transaction Signature:", signature);
  
    console.log('=========== STEP 2: init the ATA')
     const associatedTokenAccount = await getAssociatedTokenAddress(
          mint.publicKey, // mint pubkey
          payer.publicKey, // owner pubkey
          false, // allow owner off-curve
          TOKEN_PROGRAM_ID
      );
  
      // Create ATA creation instructions for all accounts
      const createAtaInstruction = createAssociatedTokenAccountIdempotentInstruction(
          payer.publicKey, // payer
          associatedTokenAccount, // associated token account address
          payer.publicKey, // owner
          mint.publicKey, // mint
          TOKEN_PROGRAM_ID
      );
  
       transaction = new Transaction().add(
          createAtaInstruction,
      );
  
      signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
  
      console.log("ATA Address:", associatedTokenAccount.toBase58());
      console.log("Transaction Signature:", signature);
  
      console.log('=========STEP 3:mint token to ATA  21 000 000')
         const mintAmount = 21 * (10 ** 6); // 假设代币有6位小数，铸造1000个代币  , 应该获取一下， mintToken 的数量 的
      const mintToInstruction =  createMintToInstruction(
          mint.publicKey,
          associatedTokenAccount,
          payer.publicKey,
          mintAmount,
          [],
          TOKEN_PROGRAM_ID
      );
  
       transaction = new Transaction().add(
          createAtaInstruction,
          mintToInstruction
      );
  
       signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
      
  
      console.log(`Minted ${mintAmount / (10 ** 6)} tokens to ATA!`);
  
  
  
    } catch (error) {
      console.error(`Oops, something went wrong: ${error}`);
    }
  }
  