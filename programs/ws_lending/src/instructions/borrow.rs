use crate::errors::ErrorCode;
use crate::states::{Bank, User};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
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

   Ok(())
}