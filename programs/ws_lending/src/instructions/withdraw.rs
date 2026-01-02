use std::f32::consts::E;

use crate::errors::ErrorCode;
use crate::states::{Bank, User};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut,seeds=[mint.key().as_ref()],bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut,seeds=[b"treasury",mint.key().as_ref()],bump)]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,seeds=[signer.key().as_ref()],bump)]
    pub user_account: Account<'info, User>,
    #[account(init_if_needed,payer = signer,associated_token::mint = mint,associated_token::authority = signer,
        associated_token::token_program = token_program)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_withdraw(ctx: Context<Withdraw>, amount_to_withdraw: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let deposited_value: u64;
    if ctx.accounts.mint.key() == user.usdc_mint_address {
        deposited_value = user.deposited_usdc;
    } else {
        deposited_value = user.deposited_sol;
    }
    /*if amount_to_withdraw > deposited_value {
        return Err(ErrorCode::InsufficientBalance.into());
    }*/
    let time_difference = user
        .last_updated
        .checked_sub(Clock::get()?.unix_timestamp)
        .unwrap();
    let bank = &mut ctx.accounts.bank;
    bank.total_deposits = (bank.total_deposits as f64
        * E.powf(bank.interest_rate as f32 * time_difference as f32) as f64)
        .round() as u64;

    let value_per_share = bank.total_deposits as f64 / bank.total_deposits_share as f64;
    let user_value = deposited_value as f64 / value_per_share;
    if amount_to_withdraw > user_value as u64 {
        return Err(ErrorCode::InsufficientBalance.into());
    }
    //Ok(())
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.bank_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.bank_token_account.to_account_info(),
    };
    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"treasury",
        mint_key.as_ref(),
        &[ctx.bumps.bank_token_account],
    ]];
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_cpi_accounts, signer_seeds);
    let decimal = ctx.accounts.mint.decimals;
    transfer_checked(cpi_ctx, amount_to_withdraw, decimal)?;
    let bank = &mut ctx.accounts.bank;
    let shares_to_withdraw =
        (amount_to_withdraw as f64 / bank.total_deposits as f64) * bank.total_deposits_share as f64;
    let user = &mut ctx.accounts.user_account;
    if ctx.accounts.mint.to_account_info().key() == user.usdc_mint_address {
        user.deposited_usdc = user.deposited_usdc.checked_sub(amount_to_withdraw).unwrap();
        user.deposited_usdc_share = user
            .deposited_usdc_share
            .checked_sub(shares_to_withdraw as u64)
            .unwrap();
    }
    bank.total_deposits = bank.total_deposits.checked_sub(amount_to_withdraw).unwrap();
    bank.total_deposits_share = bank
        .total_deposits_share
        .checked_sub(shares_to_withdraw as u64)
        .unwrap();
    Ok(())
}
