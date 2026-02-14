import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import dotenv from 'dotenv';
import bs58 from 'bs58';
import { loadPayerFromFile } from '../utils/loadPayer';

dotenv.config();

async function main() {

  const connection = new Connection(process.env.RPC_URL!, 'confirmed');

//   const secretKey = bs58.decode(process.env.PAYER_SECRET_KEY!);
//   const payer = Keypair.fromSecretKey(secretKey);

  const payer = loadPayerFromFile(process.env.PHANTOM_PAYER_FILE_PATH);  // 默认 ~/.config/solana/id.json

  // 检查余额（可选）
  const balance = await connection.getBalance(payer.publicKey);
  console.log('Payer 公钥:', payer.publicKey.toBase58());
  console.log('Payer balance:', balance / LAMPORTS_PER_SOL, 'SOL');

  if (balance < 0.01 * LAMPORTS_PER_SOL) {
    console.log('余额太低！请 airdrop: solana airdrop 2');
    return;
  }

    // 创建 Mint（代币），这个是spl-token  库提供的简化方法， 很简单， 几个参数就好

  const mint = await createMint(
    connection,
    payer,             // 支付者
    payer.publicKey,   // mint authority
    null,              // freeze authority
    9                  // decimals (USDC 常用 6，SOL 9)
  );

  console.log('Mint 创建成功:', mint.toBase58());
  console.log('Explorer:', `https://explorer.solana.com/address/${mint.toBase58()}?cluster=devnet`);
}

main().catch(console.error);