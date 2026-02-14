use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{TransferChecked, transfer_checked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors::EscrowError, state::Escrow};
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,    // 托管发起者， 决定条款， 并将mint_a 存入  escrow的用户
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,   // 托管账户， 存储托管条款

    /// Token Accounts
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,  // 托管中存入的资产类型
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>, // 托管中接收的资产类型
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,  // 托管发起者持有的， 用于存入托管的资产账户  , 即发起者的 mint_a 账户
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,   // 托管账户， 存储托管资产的账户， 由 escrow PDA 控制

    /// Programs
    pub associated_token_program: Program<'info, AssociatedToken>,  // 用于创建关联token账户 的关联代币程序
    pub token_program: Interface<'info, TokenInterface>,   // 用于CPI 转账的代币程序
    pub system_program: Program<'info, System>,   // 用于创建本Escrow（托管） 的系统程序 
}


impl<'info> Make<'info> {
    /// # Create the Escrow
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump,
        });

        Ok(())
    }

    /// # Deposit the tokens
    fn deposit_tokens(&self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount,
            self.mint_a.decimals,
        )?;

        Ok(())
    }
}


pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    // Validate the amount
    require_gt!(receive, 0, EscrowError::InvalidAmount);
    require_gt!(amount, 0, EscrowError::InvalidAmount);

    // Save the Escrow Data
    ctx.accounts.populate_escrow(seed, receive, ctx.bumps.escrow)?;

    // Deposit Tokens
    ctx.accounts.deposit_tokens(amount)?;

    Ok(())
}
