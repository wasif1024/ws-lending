use crate::errors::ErrorCode;
use crate::states::{Bank, User};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};
use crate::constants::{MAX_AGE,SOL_USD_FEED_ID,USDC_USD_FEED_ID};
use std::f32::consts::E;
#[derive(Accounts)]
pub struct Borrow<'info> {
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
    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
pub fn process_borrow(ctx: Context<Borrow>,amount_to_borrow:u64) -> Result<()> {

    let bank=&mut ctx.accounts.bank;
    let user=&mut ctx.accounts.user_account;
    let price_update=&mut ctx.accounts.price_update;
    let total_collateral:u64;
    match ctx.accounts.mint.to_account_info().key(){
        key if key==user.usdc_mint_address=>{
            let sol_feed_id=get_feed_id_from_hex(SOL_USD_FEED_ID)?;
            let sol_price=price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &sol_feed_id)?;
            let new_value=calculate_accrued_interest(user.deposited_sol,bank.interest_rate,user.last_updated)?;
            total_collateral=sol_price.price as u64 *new_value as u64;
        }
        _=>{
            let usdc_feed_id=get_feed_id_from_hex(USDC_USD_FEED_ID)?;
            let usdc_price=price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &usdc_feed_id)?;
            let new_value=calculate_accrued_interest(user.deposited_usdc,bank.interest_rate,user.last_updated)?;
            total_collateral=usdc_price.price as u64 * new_value as u64;
        }
    }
    let borrowable_amount=total_collateral.checked_mul(bank.liquidation_threshold).unwrap();
   if borrowable_amount<amount_to_borrow{
    return Err(ErrorCode::OverBorrowedLimit.into());
   }
   let transfer_cpi_accounts=TransferChecked{
    from:ctx.accounts.bank_token_account.to_account_info(),
    to:ctx.accounts.user_token_account.to_account_info(),
    mint:ctx.accounts.mint.to_account_info(),
    authority:ctx.accounts.bank_token_account.to_account_info(),
   };
   let cpi_program=ctx.accounts.token_program.to_account_info();
   let mint_key=ctx.accounts.mint.key();
   let signer_seeds: &[&[&[u8]]] = &[&[
        b"treasury",
        mint_key.as_ref(),
        &[ctx.bumps.bank_token_account],
    ]];
   let cpi_ctx=CpiContext::new_with_signer(cpi_program, transfer_cpi_accounts, signer_seeds);
   let decimal=ctx.accounts.mint.decimals;
   transfer_checked(cpi_ctx, amount_to_borrow, decimal)?;
   if bank.total_borrowed==0{
    bank.total_borrowed=amount_to_borrow;
    bank.total_borrowed_share=amount_to_borrow;
   }
   let borrow_ratio=amount_to_borrow.checked_mul(bank.total_borrowed).unwrap();
   let user_shares=bank.total_borrowed_share.checked_mul(borrow_ratio).unwrap();
   match ctx.accounts.mint.to_account_info().key(){
    key if key==user.usdc_mint_address=>{
        user.borrowed_usdc=user.borrowed_usdc.checked_add(amount_to_borrow).unwrap();
        user.borrowed_usdc_share=user.borrowed_usdc_share.checked_add(user_shares).unwrap();
    }
    _=>{
        user.borrowed_sol=user.borrowed_sol.checked_add(amount_to_borrow).unwrap();
        user.borrowed_sol_share=user.borrowed_sol_share.checked_add(user_shares).unwrap();
    }
   }
   Ok(())
}
fn calculate_accrued_interest(deposited_value:u64,interest_rate:u64,last_updated:i64)->Result<(u64)>{
    let current_time=Clock::get()?.unix_timestamp;
    let time_diff=current_time-last_updated;
    //Compound Interest
    let new_value=(deposited_value as f64 *E.powf(interest_rate as f32*time_diff as f32)as f64) as u64;
    Ok(new_value)
}