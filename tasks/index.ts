import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import {
  createMint,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from '@solana/spl-token';

async function main() {
  // 连接 Devnet（测试网）
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // 测试用临时 payer（实际项目中从钱包/环境变量加载）
  const payer = Keypair.generate();  // 记得在真实场景中替换成你的密钥！

  console.log('Payer publicKey:', payer.publicKey.toBase58());

  // 1. 创建 Mint（代币，6位小数）
  const mint = await createMint(
    connection,
    payer,               // 支付交易费 + Mint 权限
    payer.publicKey,     // Mint 权限持有者
    null,                // 冻结权限（null = 无）
    6                    // 小数位
  );
  console.log('Mint 创建成功:', mint.toBase58());

  // 2. 创建普通 Token 账户
  const tokenAccount = await createAccount(
    connection,
    payer,
    mint,
    payer.publicKey      // 账户所有者
  );
  console.log('Token Account:', tokenAccount.toBase58());

  // 3. 创建/获取 Associated Token Account（ATA，推荐方式）
  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey
  );
  console.log('Associated Token Account:', ata.address.toBase58());
}

main().catch(console.error);