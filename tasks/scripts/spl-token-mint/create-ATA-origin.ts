import {
    Connection,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
} from "@solana/web3.js";

import {
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountIdempotentInstruction,
    getAssociatedTokenAddress,
    getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

import dotenv from "dotenv";
import { checkPayerBalance, loadPayerFromFile } from "../utils/loadPayer";

dotenv.config();

async function main() {
    
    const connection = new Connection(process.env.RPC_URL!, 'confirmed');
    const payer = loadPayerFromFile(process.env.PHANTOM_PAYER_FILE_PATH);  // 默认 ~/.config/solana/id.json
    const isBalanceOk = await  checkPayerBalance(connection, payer, 0.01);
    if (!isBalanceOk) {
        return;
    }

    
    const mintAddress = process.env.MINT_TOKEN_ADDRESS!;
    if (!mintAddress) {
        throw new Error('请在 .env 文件中设置 MINT_TOKEN_ADDRESS');
    }

    const mintPublicKey = new PublicKey(mintAddress);

    // 创建ATA 开始
    const associatedTokenAccount = await getAssociatedTokenAddress(
        mintPublicKey, // mint pubkey
        payer.publicKey, // owner pubkey
        false, // allow owner off-curve
        TOKEN_PROGRAM_ID
    );

    // Create ATA creation instructions for all accounts
    const createAtaInstruction = createAssociatedTokenAccountIdempotentInstruction(
        payer.publicKey, // payer
        associatedTokenAccount, // associated token account address
        payer.publicKey, // owner
        mintPublicKey, // mint
        TOKEN_PROGRAM_ID
    );

    const transaction = new Transaction().add(
        createAtaInstruction,
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);

    console.log(`Associated Token created! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    // 创建ATA 结束

    // 也可以用以下简化的方法来创建 ATA
    // const ata = await getOrCreateAssociatedTokenAccount(
    //     connection, // connection
    //     keypair, // payer
    //     mintPublicKey, // mint pubkey
    //     keypair.publicKey // owner pubkey
    // );


}

main().catch(console.error);   // 这个才是调起main的命令， 否则命令不执行的