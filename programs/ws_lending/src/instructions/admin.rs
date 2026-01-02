use crate::states::{Bank, User};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct InitializeBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(init, payer = signer, space = 8 + Bank::INIT_SPACE, seeds = [mint.key().as_ref()], bump)]
    pub bank: Account<'info, Bank>,
    #[account(init,payer = signer,token::mint = mint,token::authority = bank_token_account,seeds = [b"treasury",mint.key().as_ref()],bump)]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init,payer=signer,space=8+User::INIT_SPACE,seeds=[signer.key().as_ref()],bump)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
pub fn process_initialize_bank(
    ctx: Context<InitializeBank>,
    liquidation_threshold: u64,
    max_ltv: u64,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    bank.authority = ctx.accounts.signer.key();
    bank.mint_address = ctx.accounts.mint.key();
    bank.liquidation_threshold = liquidation_threshold;
    bank.max_ltv = max_ltv;
    bank.interest_rate = 0.05 as u64;
    Ok(())
}
pub fn process_init_user(ctx: Context<InitUser>, usdc_mint_address: Pubkey) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.owner = ctx.accounts.signer.key();
    user.usdc_mint_address = usdc_mint_address;
    Ok(())
}
