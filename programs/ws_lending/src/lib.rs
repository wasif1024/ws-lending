use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
use instructions::admin::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::borrow::*;
use states::*;
pub mod errors;
use errors::*;
pub mod constants;

declare_id!("FDpT1vmBWwJvEf7RbDAy1STwUs4AUEXraB6wEnj5bVRd");

#[program]
pub mod ws_lending {
    use super::*;
    pub fn initialize_bank(
        ctx: Context<InitializeBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        process_initialize_bank(ctx, liquidation_threshold, max_ltv)
    }
    pub fn init_user(ctx: Context<InitUser>, usdc_mint_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_mint_address)
    }
    pub fn deposit(ctx: Context<Deposit>, amount_to_deposit: u64) -> Result<()> {
        process_deposit(ctx, amount_to_deposit)
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount_to_withdraw: u64) -> Result<()> {
        process_withdraw(ctx, amount_to_withdraw)
    }
    pub fn borrow(ctx: Context<Borrow>, amount_to_borrow: u64) -> Result<()> {
        process_borrow(ctx, amount_to_borrow)
    }
}
