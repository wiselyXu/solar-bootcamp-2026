// scripts/utils/loadPayer.ts
import { Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import fs from 'fs';
import os from 'os';
import path from 'path';

export function loadPayerFromFile(filePath?: string): Keypair {
  // 默认用 Solana CLI 的路径
  const defaultPath = path.join(os.homedir(), '.config', 'solana', 'id.json');
  const resolvedPath = filePath || defaultPath;

  if (!fs.existsSync(resolvedPath)) {
    throw new Error(`Keypair 文件不存在: ${resolvedPath}。请先用 solana-keygen new 生成，或指定路径。`);
  }

  try {
    const fileContent = fs.readFileSync(resolvedPath, 'utf-8');
    const secretKeyArray = JSON.parse(fileContent); // [number, number, ...] 64 个数字
    const secretKey = Uint8Array.from(secretKeyArray);

    // 验证长度（Solana CLI 文件是 64 字节）
    if (secretKey.length !== 64) {
      throw new Error('无效的 keypair 文件格式（应为 64 字节数组）');
    }

    return Keypair.fromSecretKey(secretKey);
  } catch (err) {
    throw new Error(`加载 keypair 失败: ${(err as Error).message}`);
  }
}


export async function checkPayerBalance(connection: any, payer: Keypair, minBalanceSOL: number = 0.01): Promise<boolean> {
      const balance = await connection.getBalance(payer.publicKey);
      console.log('Payer 公钥:', payer.publicKey.toBase58());
      console.log('Payer balance:', balance / LAMPORTS_PER_SOL, 'SOL');
    
      if (balance < minBalanceSOL* LAMPORTS_PER_SOL) {
        console.log('余额太低！请 airdrop: solana airdrop 2');
        return false;
      }

      return true;
}