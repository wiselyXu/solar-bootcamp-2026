
import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";

import {
    createInitializeAccount3Instruction,
    ACCOUNT_SIZE,
    getMinimumBalanceForRentExemptAccount,
    TOKEN_PROGRAM_ID,
    createAccount,
    getAccount,
} from "@solana/spl-token";

import dotenv from "dotenv";
import { checkPayerBalance, loadPayerFromFile } from "../utils/loadPayer";
import { assert } from "node:console";

dotenv.config();
async function main() {
    
    const connection = new Connection(process.env.RPC_URL!, 'confirmed');
    const payer = loadPayerFromFile(process.env.PHANTOM_PAYER_FILE_PATH);  // 默认 ~/.config/solana/id.json
    const isBalanceOk = await  checkPayerBalance(connection, payer, 0.01);
    if (!isBalanceOk) {
        return;
    }

    const mintPubkey = new PublicKey("3wGm7vi71y3iQwJZbuKMuJTYX9GC8nyjawr9ZW6MerHw") // 这个是 create-mint.ts 里创建的 mint 地址
    // 创建 token 账户  开始
    const token = Keypair.generate();
    const tokenRent = await getMinimumBalanceForRentExemptAccount(connection);

    const createAccountInstruction = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: token.publicKey,
        space: ACCOUNT_SIZE,
        lamports: tokenRent,
        programId: TOKEN_PROGRAM_ID
    });

    const initializeTokenInstruction = createInitializeAccount3Instruction(
        token.publicKey, // token pubkey
        mintPubkey, // mint pubkey
        payer.publicKey, // owner pubkey
        TOKEN_PROGRAM_ID
    );

    const transaction = new Transaction().add(
        createAccountInstruction,
        initializeTokenInstruction,
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [payer, token]);
    // 创建 token 账户 结束

    /***上述 创建 token 账户  开始到结束的代码 可以用简化的 函数 */
    /**
    const tokenPublickey = await createAccount(
        connection, // connection
        payer, // payer
        mintPubkey, // mint pubkey
        payer.publicKey, // owner pubkey
    );
     */
    
    console.log('Token Account 创建成功:', token.publicKey.toBase58());
    console.log(`Token created! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    const tokenAccountInfo = await getAccount(connection,token.publicKey);
    
    console.log(' which mint address can mint to this token acccount  : {}', tokenAccountInfo?.mint.toBase58());
    assert(tokenAccountInfo?.mint.toBase58() == mintPubkey.toBase58());
    console.log('Token Account belongs to owner : {}', tokenAccountInfo?.owner.toBase58());
    assert(tokenAccountInfo?.owner.toBase58() == payer.publicKey.toBase58());
    console.log('Token Account amount : {}', tokenAccountInfo?.amount.toString());
   

}

main().catch(console.error);