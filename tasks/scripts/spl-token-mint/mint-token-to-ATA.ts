import {
    Connection,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
} from "@solana/web3.js";

import {
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountIdempotentInstruction,
    createMintToInstruction,
    getAccount,
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

    // 获取ATA  将mint 出来的给到这个ata
    const ata = await getAssociatedTokenAddress(
        mintPublicKey,
        payer.publicKey,
    )    

    console.log(`ATA Address: ${ata.toBase58()}`);  

    // 构建两个指令  获取ATA 以及 mintTo  
    const createAtaInstruction = createAssociatedTokenAccountIdempotentInstruction(
        payer.publicKey, // payer
        ata, // associated token account address
        payer.publicKey, // owner
        mintPublicKey, // mint
        TOKEN_PROGRAM_ID
    );
    
    
    const mintAmount = 2000 * (10 ** 6); // 假设代币有6位小数，铸造1000个代币  , 应该获取一下， mintToken 的数量 的
    const mintToInstruction =  createMintToInstruction(
        mintPublicKey,
        ata,
        payer.publicKey,
        mintAmount,
        [],
        TOKEN_PROGRAM_ID
    );

    const transaction = new Transaction().add(
        createAtaInstruction,
        mintToInstruction
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    

    console.log(`Minted ${mintAmount / (10 ** 6)} tokens to ATA! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

    const ataInfo = await getAccount(connection, ata);

    // console.log(`ATA Info: ${JSON.stringify(ataInfo, null, 2)}`);   // 报错  to serialize a BigInt  at JSON.stringify (<anonymous>)
    console.log(`ATA Balance: ${ataInfo.amount / BigInt(10 ** 6)} tokens`);
    console.log(`ATA Address: ${ata.toBase58()}`);


    // 上面 也可能通过 简单的抽象方法来实现    mintTo  方法
/*
    const ata = await getOrCreateAssociatedTokenAccount(
        connection,
        keypair,
        mint,
        destination.publicKey
    );
    console.log(`This is your ATA: ${ata.address}!`)
      
    let tx = await mintTo(
        connection,
        keypair,
        mint,
        ata.address,
        keypair.publicKey,
        1e6,
    );
    */


}

main().catch(console.error);