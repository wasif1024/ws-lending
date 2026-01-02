use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface,TokenAccount,TransferChecked,transfer_checked};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::{Bank,User};
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut,seeds=[mint.key().as_ref()],bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut,seeds=[b"treasury",mint.key().as_ref()],bump)]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,seeds=[signer.key().as_ref()],bump)]
    pub user: Account<'info, User>,
    #[account(mut,associated_token::mint = mint,associated_token::authority = signer,associated_token::token_program = token_program)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
pub fn process_deposit(ctx: Context<Deposit>,amount_to_deposit:u64) -> Result<()> {
    let transfer_cpi_accounts = TransferChecked{
        from: ctx.accounts.user_token_account.to_account_info(),
        to:ctx.accounts.bank_token_account.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        authority:ctx.accounts.signer.to_account_info(),
    };
    let cpi_program=ctx.accounts.token_program.to_account_info();
    let cpi_ctx=CpiContext::new(cpi_program, transfer_cpi_accounts);
    let decimal=ctx.accounts.mint.decimals;
    transfer_checked(cpi_ctx, amount_to_deposit,decimal)?;
    let bank = &mut ctx.accounts.bank;
    if bank.total_deposits==0{
        bank.total_deposits=amount_to_deposit;
        bank.total_deposits_share=amount_to_deposit;
    }
    let deposit_ratio=amount_to_deposit.checked_mul(bank.total_deposits).unwrap();
    let user_shares=bank.total_deposits_share.checked_mul(deposit_ratio).unwrap();
let user = &mut ctx.accounts.user;
match ctx.accounts.mint.to_account_info().key(){
    key if key==user.usdc_mint_address=>{
        user.deposited_usdc=user.deposited_usdc.checked_add(amount_to_deposit).unwrap();
        user.deposited_usdc_share=user.deposited_usdc_share.checked_add(user_shares).unwrap();
    }
    _=>{
        user.deposited_sol=user.deposited_sol.checked_add(amount_to_deposit).unwrap();
        user.deposited_sol_share=user.deposited_sol_share.checked_add(user_shares).unwrap();
    }
}
bank.total_deposits+=amount_to_deposit;
    Ok(())
}