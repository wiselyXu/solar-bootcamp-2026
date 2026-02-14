import { createAssociatedTokenAccountIdempotentInstruction, createTransferInstruction, getAssociatedTokenAddress } from "@solana/spl-token";
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { loadPayerFromFile } from "../utils/loadPayer";
import dotenv from "dotenv";

dotenv.config();

async function main() {
    const destination = Keypair.generate();
    const payer = loadPayerFromFile();  // 默认 ~/.config/solana/id.json
    const connection = new  Connection(process.env.RPC_URL!, 'confirmed');


    const mintPublicKey  = new PublicKey(process.env.MINT_TOKEN_ADDRESS!);

    const destinationTokenAccount = await getAssociatedTokenAddress(
        mintPublicKey,
        destination.publicKey,
    );

    const sourceTokenAccount = await getAssociatedTokenAddress(
        payer.publicKey,
        destination.publicKey,
    );

    // Create ATA creation instruction
    const createAtaInstruction = createAssociatedTokenAccountIdempotentInstruction(
        payer.publicKey, // payer
        destinationTokenAccount, // associated token account address
        destination.publicKey, // owner
        mintPublicKey, // mint
    );

    // Transfer tokens to ATA 
    const transferInstruction = createTransferInstruction(
        sourceTokenAccount, // source token account pubkey
        destinationTokenAccount, // destination token account pubkey
        payer.publicKey, // owner of the source token account
        100e6, // amount    
    );

    const transaction = new Transaction().add(
        createAtaInstruction,
        transferInstruction,
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);

    console.log(`Token accounts created and tokens transferred! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    
}

main().catch(console.error);