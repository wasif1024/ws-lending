use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct User {
    pub owner: Pubkey,
    pub deposited_sol: u64,
    pub deposited_sol_share: u64,
    pub borrowed_sol: u64,
    pub borrowed_sol_share: u64,
    pub deposited_usdc: u64,
    pub deposited_usdc_share: u64,
    pub borrowed_usdc: u64,
    pub borrowed_usdc_share: u64,
    pub usdc_mint_address: Pubkey,
    pub last_updated: i64,
}
#[account]
#[derive(InitSpace, Debug)]
pub struct Bank {
    pub authority: Pubkey,
    pub mint_address: Pubkey,
    pub total_deposits: u64,
    pub total_deposits_share: u64,
    pub total_borrowed: u64,
    pub total_borrowed_share: u64,
    pub liquidation_threshold: u64, //The loan to value defined so that loan can be liquidated
    pub liquidation_bonus: u64, //Percentage of the bonus will be sent to liquidator for processing the liquidation
    pub liquidation_close_factor: u64, //Percentage of the collateral that can be liquidated
    pub max_ltv: u64, //Max percentage of collateral that can be borrowed for a specific asset
    pub last_updated: i64,
    pub interest_rate: u64,
}
