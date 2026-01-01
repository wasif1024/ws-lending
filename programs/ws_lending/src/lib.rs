use anchor_lang::prelude::*;
pub mod states;
pub mod instructions;
use instructions::admin::*;
use states::*;
declare_id!("FDpT1vmBWwJvEf7RbDAy1STwUs4AUEXraB6wEnj5bVRd");

#[program]
pub mod ws_lending {
    use super::*;
pub fn initialize_bank(ctx: Context<InitializeBank>,liquidation_threshold:u64,max_ltv:u64) -> Result<()> {
    process_initialize_bank(ctx,liquidation_threshold,max_ltv)
}
pub fn init_user(ctx: Context<InitUser>,usdc_mint_address:Pubkey) -> Result<()> {
    process_init_user(ctx,usdc_mint_address)
}
}
