// #![no_std]
// #![feature(alloc_error_handler)]

// extern crate alloc;

//use alloc::vec::Vec;
use pinocchio::{
    AccountView, Address, ProgramResult, cpi::{Seed, invoke, invoke_signed}, entrypoint, error::ProgramError, instruction::InstructionView
};
use solana_program::{
    instruction::{AccountMeta, Instruction}, msg
};
use pinocchio_token::instructions::{TransferChecked, transfer_checked};

const DISCRIMINATOR_MAKE: u8 = 0;
const DISCRIMINATOR_TAKE: u8 = 1;
const DISCRIMINATOR_REFUND: u8 = 2;

const ESCROW_SEED_PREFIX: &[u8] = b"escrow";

// escrow 数据结构（手动序列化）
const ESCROW_SEED_OFFSET: usize = 0;
const ESCROW_MAKER_OFFSET: usize = 8;
const ESCROW_MINT_A_OFFSET: usize = 40;
const ESCROW_MINT_B_OFFSET: usize = 72;
const ESCROW_RECEIVE_OFFSET: usize = 104;
const ESCROW_BUMP_OFFSET: usize = 112;
const ESCROW_DATA_LEN: usize = 113; // 到 bump 结束

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        msg!("Empty instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }

    let discriminator = instruction_data[0];
    let rest_data = &instruction_data[1..];

    match discriminator {
        DISCRIMINATOR_MAKE => make(program_id, accounts, rest_data),
       // DISCRIMINATOR_TAKE => take(program_id, accounts, rest_data),
       // DISCRIMINATOR_REFUND => refund(program_id, accounts, rest_data),
        _ => {
            msg!("Unknown discriminator: {}", discriminator);
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

// ======================================================
// Make: 创建 escrow + vault ATA + 存入 token A
// 期望 accounts 顺序：
// 0: maker (signer, payer)
// 1: escrow (PDA, to be created)
// 2: mint_a
// 3: mint_b
// 4: maker_ata_a (source)
// 5: vault (ATA to be created, authority = escrow)
// 6: associated_token_program
// 7: token_program
// 8: system_program (可选，用于创建 escrow)
// ======================================================
fn make(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 24 { // seed(8) + receive(8) + amount(8)
        msg!("Make: insufficient data");
        return Err(ProgramError::InvalidInstructionData);
    }

    let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

    if amount == 0 || receive == 0 {
        msg!("Make: amount or receive is zero");
        return Err(ProgramError::InvalidArgument);
    }

    if accounts.len() < 8 {
        msg!("Make: not enough accounts");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let maker = &accounts[0];
    let escrow = &accounts[1];
    let mint_a = &accounts[2];
    let mint_b = &accounts[3];
    let maker_ata_a = &accounts[4];
    let vault = &accounts[5];
    let ata_program = &accounts[6];
    let token_program = &accounts[7];

    if !maker.is_signer() {
        msg!("Maker is not signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 计算 escrow PDA
    let seeds = &[&ESCROW_SEED_PREFIX[..], maker.address().as_ref(), &seed.to_le_bytes()[..]];
    let (expected_pda, bump) = Address::find_program_address(seeds, program_id);

    if *escrow.address() != expected_pda {
        msg!("Escrow PDA mismatch");
        return Err(ProgramError::InvalidAccountData);
    }

    // 如果 escrow 未初始化，假设客户端已用 system program 创建，这里只写数据
    if escrow.data_len() == 0 {
        msg!("Escrow account not initialized by client");
        return Err(ProgramError::InvalidAccountData);
    }

    // 写 escrow 数据（手动序列化）
    let mut escrow_data = escrow.try_borrow_mut_data()?;
    if escrow_data.len() < ESCROW_DATA_LEN {
        msg!("Escrow data too small");
        return Err(ProgramError::InvalidAccountData);
    }

    escrow_data[ESCROW_SEED_OFFSET..ESCROW_SEED_OFFSET+8].copy_from_slice(&seed.to_le_bytes());
    escrow_data[ESCROW_MAKER_OFFSET..ESCROW_MAKER_OFFSET+32].copy_from_slice(maker.address().as_ref());
    escrow_data[ESCROW_MINT_A_OFFSET..ESCROW_MINT_A_OFFSET+32].copy_from_slice(mint_a.address().as_ref());
    escrow_data[ESCROW_MINT_B_OFFSET..ESCROW_MINT_B_OFFSET+32].copy_from_slice(mint_b.address().as_ref());
    escrow_data[ESCROW_RECEIVE_OFFSET..ESCROW_RECEIVE_OFFSET+8].copy_from_slice(&receive.to_le_bytes());
    escrow_data[ESCROW_BUMP_OFFSET] = bump;

    // 转移 token A 到 vault
    transfer_checked(
        maker.address(),
        maker_ata_a.address(),
        mint_a.address(),
        vault.address(),
        maker.address(),
        amount,
        mint_a.decimals(),
        token_program.address(),
    )?;

    msg!("Make success - seed: {}, amount: {}", seed, amount);

    Ok(())
}

// ======================================================
// Take & Refund 逻辑类似 Anchor 版本，但用 pinocchio 的方式写 CPI
// 这里省略完整实现（太长），核心是：
// 1. 从 escrow 读取 seed/bump/maker/mint_a/mint_b/receive
// 2. 验证 has_one 等约束
// 3. transfer_checked CPI + PDA 签名（用 invoke_signed）
// 4. close_account CPI + PDA 签名
// ======================================================

// 为了篇幅，take 和 refund 的完整实现可以参考之前的 Anchor 逻辑，手动反序列化 escrow 数据，然后构建 TransferChecked 和 CloseAccount CPI。

// 示例（take 的核心部分）：
/*
fn take(...) -> ProgramResult {
    // 读取 escrow 数据
    let escrow_data = escrow.try_borrow_data()?;
    let escrow_seed = u64::from_le_bytes(escrow_data[0..8].try_into().unwrap());
    let escrow_bump = escrow_data[112];

    let seeds = &[
        ESCROW_SEED_PREFIX,
        maker.address().as_ref(),
        &escrow_seed.to_le_bytes(),
        &[escrow_bump],
    ];

    // 转移 token B (taker -> maker)
    transfer_checked(...)?.invoke()?;

    // PDA 签名转移 token A (vault -> taker)
    invoke_signed(
        &transfer_checked(
            escrow.address(),
            vault.address(),
            mint_a.address(),
            taker_ata_a.address(),
            escrow.address(),
            vault.lamports(),
            mint_a.decimals(),
            token_program.address(),
        )?,
        &[escrow.clone(), vault.clone(), taker_ata_a.clone(), mint_a.clone(), token_program.clone()],
        &[seeds],
    )?;

    // 关闭 vault
    invoke_signed(
        &close_account(
            vault.address(),
            maker.address(),
            escrow.address(),
            token_program.address(),
        )?,
        &[vault.clone(), maker.clone(), escrow.clone(), token_program.clone()],
        &[seeds],
    )?;

    Ok(())
}
*/




//  -----------
fn transfer_checked_cpi(
    from_token_account: &AccountView,
    mint: &AccountView,
    to_token_account: &AccountView,
    authority: &AccountView,
    amount: u64,
    decimals: u8,
    token_program: &AccountView,
) -> ProgramResult {
    // TransferChecked discriminator = 12 (SPL Token Program)
    let mut data = vec![12u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    let accounts = vec![
        AccountMeta::new(*from_token_account.address(), false),
        AccountMeta::new_readonly(*mint.address(), false),
        AccountMeta::new(*to_token_account.address(), false),
        AccountMeta::new_readonly(*authority.address(), true),
    ];

    let ix = Instruction {
        program_id: *token_program.address(),
        accounts,
        data,
    };

    invoke(&ix, &[
        from_token_account.clone(),
        mint.clone(),
        to_token_account.clone(),
        authority.clone(),
        token_program.clone(),
    ])?;

    Ok(())
}


fn transfer_checked_pda(
    from_token_account: &AccountView,
    mint: &AccountView,
    to_token_account: &AccountView,
    authority: &AccountView,  // PDA
    amount: u64,
    decimals: u8,
    token_program: &AccountView,
    seeds: &[&[u8]],
) -> ProgramResult {
    let mut data = vec![12u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    let accounts = vec![
        AccountMeta::new(*from_token_account.address(), false),
        AccountMeta::new_readonly(*mint.address(), false),
        AccountMeta::new(*to_token_account.address(), false),
        AccountMeta::new_readonly(*authority.address(), true),
    ];

    let ix = Instruction {
        program_id: *token_program.address(),
        accounts,
        data,
    };

    invoke_signed(&ix, &[from_token_account.clone(), mint.clone(), to_token_account.clone(), authority.clone(), token_program.clone()], &[seeds])?;

    Ok(())
}