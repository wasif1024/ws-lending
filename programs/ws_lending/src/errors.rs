use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Over borrowed limit")]
    OverBorrowedLimit,
    #[msg("Over repay amount")]
    OverRepayAmount,
}
