import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";

import {
    createInitializeMint2Instruction,
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import dotenv from "dotenv";
import { loadPayerFromFile } from "../utils/loadPayer";

dotenv.config();
console.log('check .env variables...');
if (!process.env.RPC_URL) {
      throw new Error('请在 .env 文件中设置 RPC_URL');
}else {
  console.log('RPC_URL:', process.env.RPC_URL);
}

if (!process.env.PHANTOM_PAYER_FILE_PATH) {
      throw new Error('请在 .env 文件中设置 PHANTOM_PAYER_FILE_PATH');
}else {
  console.log('PHANTOM_PAYER_FILE_PATH:', process.env.PHANTOM_PAYER_FILE_PATH);
}

async function main() {

    const connection = new Connection(process.env.RPC_URL!, 'confirmed');

    //   const secretKey = bs58.decode(process.env.PAYER_SECRET_KEY!);
    //   const payer = Keypair.fromSecretKey(secretKey);
    
      const payer = loadPayerFromFile();  // 默认 ~/.config/solana/id.json
    
      // 检查余额（可选）
      const balance = await connection.getBalance(payer.publicKey);
      console.log('Payer 公钥:', payer.publicKey.toBase58());
      console.log('Payer balance:', balance / LAMPORTS_PER_SOL, 'SOL');
    
      if (balance < 0.01 * LAMPORTS_PER_SOL) {
        console.log('余额太低！请 airdrop: solana airdrop 2');
        return;
      }

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

    const transaction = new Transaction().add(
        createAccountInstruction,
        initializeMintInstruction,
    );

    // mint  是新生成的keypair，需要签名, 来拥有  这个新建 的 mint 账户, (p j   System Program createAccount 的要求： 新账户的owner  需要签名)
    // payer 需要签名， 因为  payer 是  需要去支付  rent-exempt的 lamports 和交易费
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer, mint]);

    console.log(`Mint created! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

}



main().catch(console.error);