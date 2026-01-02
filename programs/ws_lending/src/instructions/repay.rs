use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked,transfer_checked};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::{Bank, User};
use std::f32::consts::E;
use crate::errors::ErrorCode;
#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut,seeds=[mint.key().as_ref()],bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut,seeds=[b"treasury",mint.key().as_ref()],bump)]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,seeds=[signer.key().as_ref()],bump)]
    pub user_account: Account<'info, User>,
    #[account(mut,associated_token::mint = mint,associated_token::authority = signer,associated_token::token_program = token_program)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
pub fn process_repay(ctx: Context<Repay>, amount_to_repay: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let borrowed_value:u64;
    match ctx.accounts.mint.to_account_info().key(){
        key if key==user.usdc_mint_address=>{
            borrowed_value=user.borrowed_usdc;
        }
        _=>{
            borrowed_value=user.borrowed_sol;
        }
    }
    let time_difference=user.last_update_borrow.checked_sub(Clock::get()?.unix_timestamp).unwrap();
    let bank=&mut ctx.accounts.bank;
    bank.total_borrowed=(bank.total_borrowed as f64 * E.powf(bank.interest_rate as f32 * time_difference as f32) as f64).round() as u64;
    let value_per_share=bank.total_borrowed as f64/bank.total_borrowed_share as f64;
    let user_value=borrowed_value as f64/value_per_share;
    if amount_to_repay > user_value as u64{
        return Err(ErrorCode::OverRepayAmount.into());
    }
    let transfer_cpi_accounts=TransferChecked{
        from:ctx.accounts.user_token_account.to_account_info(),
        to:ctx.accounts.bank_token_account.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        authority:ctx.accounts.user_token_account.to_account_info(),
    };
    let cpi_program=ctx.accounts.token_program.to_account_info();
    let cpi_ctx=CpiContext::new(cpi_program, transfer_cpi_accounts);
    let decimal=ctx.accounts.mint.decimals;
    transfer_checked(cpi_ctx, amount_to_repay, decimal)?;
    let borrow_ratio=amount_to_repay.checked_mul(bank.total_borrowed).unwrap();
    let user_shares=bank.total_borrowed_share.checked_mul(borrow_ratio).unwrap();
    match ctx.accounts.mint.to_account_info().key(){
        key if key==user.usdc_mint_address=>{
            user.borrowed_usdc=user.borrowed_usdc.checked_sub(amount_to_repay).unwrap();
            user.borrowed_usdc_share=user.borrowed_usdc_share.checked_sub(user_shares).unwrap();
        }
        _=>{
            user.borrowed_sol=user.borrowed_sol.checked_sub(amount_to_repay).unwrap();
            user.borrowed_sol_share=user.borrowed_sol_share.checked_sub(user_shares).unwrap();
        }
    }
    bank.total_borrowed=bank.total_borrowed.checked_sub(amount_to_repay).unwrap();
    bank.total_borrowed_share=bank.total_borrowed_share.checked_sub(user_shares).unwrap();
    Ok(())
}