use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};
use solana_address::declare_id;

// use crate::instructions::VaultContext;

// mod instructions;
// declare_id!("22222222222222222222222222222222222222222222");

// entrypoint!(process_instruction);
// fn process_instruction(
//     _program_id: &Address,
//     accounts: &[AccountView],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     // Your program logic goes here
//     let (discriminator, instrcution_data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
//     match *discriminator {
//         0 => VaultContext::try_from((accounts, instrcution_data, *discriminator))?.deposit()?,
//         1 => VaultContext::try_from((accounts, instrcution_data, *discriminator))?.withdraw()?,
//         _ => return Err(ProgramError::InvalidInstructionData),
//     }  

//     Ok(())
// }


entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;
pub mod error;
pub use error::*;

declare_id!("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Make::DISCRIMINATOR, data)) => Make::try_from((data, accounts))?.process(),
        Some((Take::DISCRIMINATOR, _)) => Take::try_from(accounts)?.process(),
        Some((Refund::DISCRIMINATOR, _)) => Refund::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}