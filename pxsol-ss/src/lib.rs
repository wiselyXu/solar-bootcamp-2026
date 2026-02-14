// // 必须放在最前面
// #![cfg_attr(target_arch = "bpf", allow(unused))]

// // 定义 custom backend 模块
// #[cfg(all(
//     target_arch = "bpf",
//     getrandom_backend = "custom"
// ))]
// mod custom_backend {
//     use core::num::NonZeroU32;

//     // getrandom 0.3.4 需要的 trait
//     pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), getrandom::Error> {
//         // 简单实现
//         for byte in dest {
//             *byte = 0;
//         }
//         Ok(())
//     }

//     // 注册 backend
//     #[no_mangle]
//     pub extern "C" fn __getrandom_custom_getrandom_inner(
//         dest: *mut u8,
//         len: usize,
//     ) -> i32 {
//         unsafe {
//             let slice = core::slice::from_raw_parts_mut(dest, len);
//             match getrandom_inner(slice) {
//                 Ok(()) => 0,
//                 Err(e) => e.raw_os_error().unwrap_or(1) as i32,
//             }
//         }
//     }
// }

// // 导出必要的符号
// #[cfg(all(
//     target_arch = "bpf",
//     getrandom_backend = "custom"
// ))]
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn getrandom_custom_backend() -> *const u8 {
//     b"custom\0".as_ptr()
// }

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, solana!");
    msg!(" update from original hello world example");
    msg!("Generating a random u128 number using getrandom crate...");

    match get_random_u128() {
        Ok(random_number) => {
            msg!("Generated random u128 number: {}", random_number);
        }
        Err(e) => {
            msg!("Generated random u128 number error: {}", e);
        }
    }
    Ok(())
}

fn get_random_u128() -> Result<u128, getrandom::Error> {
    let mut buf = [0u8; 16];
    //getrandom::getrandom(&mut buf)?;
    getrandom::fill(&mut buf)?;
    Ok(u128::from_le_bytes(buf))
}

#[cfg(test)]
mod test {
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::Instruction,
        message::Message,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[test]
    fn test_pxsol_ss() {
        // Create a new LiteSVM instance
        let mut svm = LiteSVM::new();

        // Create a keypair for the transaction payer
        let payer = Keypair::new();

        // Airdrop some lamports to the payer
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        // Load our program
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();
        svm.add_program_from_file(program_id, "target/deploy/pxsol_ss.so")
            .unwrap();

        // Create instruction with no accounts and no data
        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        };

        // Create transaction
        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(&[&payer], message, svm.latest_blockhash());

        // Send transaction and verify it succeeds
        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Transaction should succeed");
        let logs = result.unwrap().logs;
        println!("Logs: {logs:#?}");
    }
}
