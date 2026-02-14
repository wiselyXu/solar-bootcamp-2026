use pinocchio::{AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError};
use pinocchio_associated_token_account::instructions::Create;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::{InitializeAccount3, InitializeMint2};
use solana_program::{rent::Rent, sysvar::Sysvar, pubkey::Pubkey};

// Import the find_program_address function
//use solana_sdk::pubkey::Pubkey::find_program_address;


// Import the find_program_address function
// Define a Trait
pub trait AccountCheck {
    fn check(account: &AccountView) -> Result<(), ProgramError>;
}

// Define a Type
pub struct SignerAccount;

// Implement the trait for different Types
impl AccountCheck for SignerAccount {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_signer() {
            
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

pub struct SystemAccount;

impl AccountCheck for SystemAccount {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        Ok(())
    }
}

//  mint account , token account
pub struct MintAccount;

impl AccountCheck for MintAccount {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if account.data_len() != pinocchio_token::state::Mint::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub trait MintInit {
    fn init(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult;
    fn init_if_needed(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult;
}

impl MintInit for MintAccount {
    fn init(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult {
        // Get required lamports for rent
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(pinocchio_token::state::Mint::LEN);

        // Fund the account with the required lamports
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::Mint::LEN as u64,
            owner: &pinocchio_token::ID,
        }.invoke()?;

        InitializeMint2 {
            mint: account,
            decimals,
            mint_authority,
            freeze_authority,
        }.invoke()
    }

    fn init_if_needed(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }
}

pub struct TokenAccount;

impl AccountCheck for TokenAccount {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::InvalidOwner.into());
        }

        if account.data_len().ne(&pinocchio_token::state::TokenAccount::LEN) {
            return Err(ProgramError::InvalidAccountData.into());
        }

        Ok(())
    }
}

pub trait AccountInit {
    fn init(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult;
    fn init_if_needed(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult;
}

impl AccountInit for TokenAccount {
    fn init(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(pinocchio_token::state::TokenAccount::LEN);

        // Fund the account with the required lamports
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::TokenAccount::LEN as u64,
            owner: &pinocchio_token::ID,
        }.invoke()?;

        // Initialize the Token Account
        InitializeAccount3 {
            account,
            mint,
            owner,
        }.invoke()
    }

    fn init_if_needed(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner),
        }
    }
}


// TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
pub const TOKEN_2022_PROGRAM_ID: [u8; 32] = [
    0x06, 0xdd, 0xf6, 0xe1, 0xee, 0x75, 0x8f, 0xde, 0x18, 0x42, 0x5d, 0xbc, 0xe4, 0x6c, 0xcd, 0xda,
    0xb6, 0x1a, 0xfc, 0x4d, 0x83, 0xb9, 0x0d, 0x27, 0xfe, 0xbd, 0xf9, 0x28, 0xd8, 0xa1, 0x8b, 0xfc,
];

const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

pub struct Mint2022Account;

impl AccountCheck for Mint2022Account {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            return Err(ProgramError::ILLEGAL_OWNER);
        }

        let data = account.try_borrow_data()?;

        if data.len().ne(&pinocchio_token::state::Mint::LEN) {
            if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                return Err(ProgramError::InvalidAccountData.into());
            }
            if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
                return Err(ProgramError::InvalidAccountData.into());
            }
        }

        Ok(())
    }
}

impl MintInit for Mint2022Account {
    fn init(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(pinocchio_token::state::Mint::LEN);

        // Fund the account with the required lamports
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::Mint::LEN as u64,
            owner: &Address::new(TOKEN_2022_PROGRAM_ID),
        }.invoke()?;

        InitializeMint2 {
            mint: account,
            decimals,
            mint_authority,
            freeze_authority,
        }.invoke()
    }

    fn init_if_needed(account: &AccountView, payer: &AccountView, decimals: u8, mint_authority: &Address, freeze_authority: Option<&Address>) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }
}
pub struct TokenAccount2022Account;

impl AccountCheck for TokenAccount2022Account {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            return Err(ProgramError::InvalidOwner.into());
        }

        let data = account.try_borrow_data()?;

        if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
            if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                return Err(ProgramError::InvalidAccountData.into());
            }
            if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR) {
                return Err(ProgramError::InvalidAccountData.into());
            }
        }

        Ok(())
    }
}

impl AccountInit for TokenAccount2022Account {
    fn init(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(pinocchio_token::state::TokenAccount::LEN);

        // Fund the account with the required lamports
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::TokenAccount::LEN as u64,
            owner: &Address::new(TOKEN_2022_PROGRAM_ID),
        }.invoke()?;

        InitializeAccount3 {
            account,
            mint,
            owner,
        }.invoke()
    }

    fn init_if_needed(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &Address) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner),
        }
    }
}


pub struct MintInterface;

impl AccountCheck for MintInterface {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            if !account.is_owned_by(&pinocchio_token::ID) {
                return Err(ProgramError::IllegalOwner);
            } else {
                if account.data_len().ne(&pinocchio_token::state::Mint::LEN) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        } else {
            let data = account.try_borrow_data()?;

            if data.len().ne(&pinocchio_token::state::Mint::LEN) {
                if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
                if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        }

        Ok(())
    }
}

pub struct TokenAccountInterface;

impl AccountCheck for TokenAccountInterface {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            if !account.is_owned_by(&pinocchio_token::ID) {
                return Err(ProgramError::InvalidOwner.into());
            } else {
                if account.data_len().ne(&pinocchio_token::state::TokenAccount::LEN) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        } else {
            let data = account.try_borrow_data()?;

            if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
                if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
                if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET]
                    .ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR)
                {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        }

        Ok(())
    }
}

pub struct AssociatedTokenAccount;

// Define the AssociatedTokenAccountCheck trait
pub trait AssociatedTokenAccountCheck {
    fn check(
        account: &AccountView,
        authority: &AccountView,
        mint: &AccountView,
        token_program: &AccountView,
    ) -> Result<(), ProgramError>;
}

impl AssociatedTokenAccountCheck for AssociatedTokenAccount {
    fn check(
        account: &AccountView,
        authority: &AccountView,
        mint: &AccountView,
        token_program: &AccountView,
    ) -> Result<(), ProgramError> {
        TokenAccount::check(account)?;

        if Address::find_program_address(
            &[authority.key(), token_program.key(), mint.key()],
            &pinocchio_associated_token_account::ID,
        )
        .0
        .ne(account.key())
        {
            return Err(ProgramError::InvalidAddress.into());
        }

        Ok(())
    }
}

pub trait AssociatedTokenAccountInit {
    fn init(
        account: &AccountView,
        mint: &AccountView,
        payer: &AccountView,
        owner: &AccountView,
        system_program: &AccountView,
        token_program: &AccountView,
    ) -> ProgramResult;

    fn init_if_needed(
        account: &AccountView,
        mint: &AccountView,
        payer: &AccountView,
        owner: &AccountView,
        system_program: &AccountView,
        token_program: &AccountView,
    ) -> ProgramResult;
}

impl AssociatedTokenAccountInit for AssociatedTokenAccount {
    fn init(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &AccountView, system_program: &AccountView, token_program: &AccountView) -> ProgramResult {
        Create {
            funding_account: payer,
            account,
            wallet: owner,
            mint,
            system_program,
            token_program,
        }.invoke()
    }

    fn init_if_needed(account: &AccountView, mint: &AccountView, payer: &AccountView, owner: &AccountView, system_program: &AccountView, token_program: &AccountView) -> ProgramResult {
        match Self::check(account, payer, mint, token_program) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner, system_program, token_program),
        }
    }
}

pub struct ProgramAccount;

impl AccountCheck for ProgramAccount {
    fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.owned_by(&crate::ID) {
            return Err(ProgramError::IllegalOwner.into());
        }

        // 这个暂时不检查 ， ProgramAccount，不知道哪个
        // if account.data_len().ne(&crate::state::ProgramAccount::LEN) {
        //     return Err(PinocchioError::InvalidAccountData.into());
        // }

        Ok(())
    }
}

pub trait ProgramAccountInit {
    fn init<'a, T: Sized>(
        payer: &AccountView,
        account: &AccountView,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult;
}

impl ProgramAccountInit for ProgramAccount {
    fn init<'a, T: Sized>(
        payer: &AccountView,
        account: &AccountView,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(space);

        // Create signer with seeds slice
        let signer = [Signer::from(seeds)];

        // Create the account
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signer)?;

        Ok(())
    }
}

pub trait AccountClose {
    fn close(account: &AccountView, destination: &AccountView) -> ProgramResult;
}

impl AccountClose for ProgramAccount {
    fn close(account: &AccountView, destination: &AccountView) -> ProgramResult {
        {
            let mut data = account.try_borrow_mut()?;
            data[0] = 0xff;
        }

        *destination.try_borrow_mut()? += *account.try_borrow_lamports()?;
        account.try_borrow_mut_data()?.resize(1, 0);
        account.close()
    }
}

#[repr(C)]
pub struct AccountExample {
    pub seed: u64,
    pub bump: [u8; 1]
}

impl AccountExample {
    /// The length of the `AccountExample` account data.
    pub const LEN: usize = size_of::<u64>() + size_of::<[u8; 1]>();

    /// Return an `AccountExample` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountView`, safe borrowing
    /// the account data.
    #[inline]
    pub fn from_account_info(account_info: &AccountView) -> Result<Ref<AccountExample>, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes(data)
        }))
    }

    /// Return a `AccountExample` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountView`, but does not
    /// perform the borrow check.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it is safe to borrow the account data – e.g., there are
    /// no mutable borrows of the account data.
    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountView,
    ) -> Result<&Self, ProgramError> {
        if account_info.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self::from_bytes(account_info.borrow_data_unchecked()))
    }

    /// Return a `AccountExample` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `AccountExample`.
    #[inline(always)]
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const AccountExample)
    }


    #[inline(always)]
    pub fn set_inner(&mut self, seed: u64, bump: [u8;1]) {
        self.seed = seed;
        self.bump = bump;
}

const SEED_OFFSET: usize = 0;

#[inline(always)]
pub fn check_program_id_and_discriminator(
    account_info: &AccountView,
) -> Result<(), ProgramError> {
    // Check Program ID
    if unsafe { account_info.owner().ne(&crate::ID) } {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check length
    if account_info.data_len().ne(Self::LEN) {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

#[inline(always)]
pub fn get_seeds(account_info: &AccountView) -> Result<u64, ProgramError> {
    Self::check_program_id_and_discriminator(account_info);

    let data = account_info.try_borrow_data()?;
    Ok(u64::from_le_bytes(data[SEED_OFFSET..SEED_OFFSET + size_of::<u64>()].try_into().unwrap()))
}

#[inline(always)]
pub unsafe fn get_seeds_unchecked(account_info: &AccountView) -> Result<u64, ProgramError> {
    let data = account_info.try_borrow_data()?;
    Ok(u64::from_le_bytes(data[SEED_OFFSET..SEED_OFFSET + size_of::<u64>()].try_into().unwrap()))
}

    #[inline(always)]
    pub fn set_seeds(account_info: &AccountView, seed: u64) -> Result<(), ProgramError> {
        Self::check_program_id_and_discriminator(account_info);

        let data = account_info.try_borrow_mut_data()?;
        Ok(unsafe {
            *(data.as_mut_ptr() as *mut [u8; 8]) = seed.to_le_bytes();
        })
    }

    #[inline(always)]
    pub fn set_seeds_unchecked(account_info: &AccountView, seed: u64) -> Result<(), ProgramError> {
        let data = account_info.try_borrow_mut_data()?;
        Ok(unsafe {
            *(data.as_mut_ptr() as *mut [u8; 8]) = seed.to_le_bytes();
        })
    }
}

