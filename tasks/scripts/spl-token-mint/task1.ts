import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
    LAMPORTS_PER_SOL,
  } from '@solana/web3.js';
  import {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    createInitializeMint2Instruction,
    createAssociatedTokenAccountIdempotentInstruction,
    createMintToInstruction,
    getAssociatedTokenAddressSync,
    createMintToCheckedInstruction,
  } from '@solana/spl-token';
  import dotenv from 'dotenv';
  import { loadPayerFromFile } from '../utils/loadPayer';  // 用你之前的 utils
  
  dotenv.config();
  
  async function main() {
    const connection = new Connection(
      process.env.RPC_URL!,
      'confirmed'
    );
  
    const payer = loadPayerFromFile();  // 你的 feePayer / mint authority Keypair
  
    console.log('Payer:', payer.publicKey.toBase58());
  
    // 检查余额（至少需要 ~0.005 SOL 用于 rent + fee）
    const balance = await connection.getBalance(payer.publicKey);
    if (balance < 0.005 * LAMPORTS_PER_SOL) {
      console.error('余额不足！运行: solana airdrop 2');
      return;
    }
  
    // Step 1: 创建 Mint account（随机 Keypair）
    const mint = Keypair.generate();
  
    const mintRent = await getMinimumBalanceForRentExemptMint(connection);
  
    const createMintAccountIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: MINT_SIZE,
      lamports: mintRent,
      programId: TOKEN_PROGRAM_ID,
    });
  
    // Step 2: 初始化 Mint（decimals=6, mintAuthority = payer, freezeAuthority = null）
    const initializeMintIx = createInitializeMint2Instruction(
      mint.publicKey,          // mint
      6,                       // decimals
      payer.publicKey,         // mint authority
      payer.publicKey,                    // freeze authority = null
      TOKEN_PROGRAM_ID
    );
  
    // Step 3: 计算并创建 ATA（幂等，针对 payer 自己的 ATA）
    const ata = getAssociatedTokenAddressSync(
      mint.publicKey,
      payer.publicKey,
      false,                   // allowOwnerOffCurve = false (默认)
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
  
    const createAtaIx = createAssociatedTokenAccountIdempotentInstruction(
      payer.publicKey,         // payer
      ata,                     // ATA 地址
      payer.publicKey,         // owner = payer
      mint.publicKey,          // mint
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
  
    // Step 4: Mint 21,000,000 tokens（人类可读数量，raw = 21_000_000 * 10^6）
    const amountHuman = 21_000_000n;
    const decimals = 6;
    const amountRaw = amountHuman * (10n ** 6n);// 21e6 * 1e6 = 21e12
  
    const mintToIx = createMintToCheckedInstruction(
      mint.publicKey,          // mint
      ata,                     // destination (ATA)
      payer.publicKey,         // mint authority
      amountRaw,               // raw amount (BigInt)
      6,                       // decimals (必须匹配 mint 的 decimals)
      [],                      // multiSigners (空数组)
      TOKEN_PROGRAM_ID
    );
    // const mintToIx = createMintToInstruction(
    //   mint.publicKey,          // mint
    //   ata,                     // destination (你的 ATA)
    //   payer.publicKey,         // mint authority
    //   amountRaw,            // raw amount (BigInt)
    //   [],                      // multiSigners (空)
    //   TOKEN_PROGRAM_ID
    // );
  
    // 打包到一个 Transaction
    const transaction = new Transaction().add(
      createMintAccountIx,
      initializeMintIx,
      createAtaIx,
      mintToIx
    );
  
    // 签名者：payer（fee + authority） + mint（创建新账户需要签名）
    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer, mint],  // 必须包含 mint Keypair！
      { skipPreflight: false, preflightCommitment: 'confirmed' }
    );
  
    console.log('挑战完成！交易签名:', signature);
    console.log('Mint 地址:', mint.publicKey.toBase58());
    console.log('你的 ATA:', ata.toBase58());
    console.log('查看交易:', `https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    console.log(`已向 ATA 铸造 ${amountHuman} 个 token (decimals=${decimals})`);
  }
  
  main().catch(console.error);