use anchor_lang::prelude::*;

// Định nghĩa các lỗi sẽ xảy ra trong chương trình
#[error_code]
pub enum AppError {
    #[msg("Tokens are already staked")]
    IsStaked,

    #[msg("Tokens are not staked")]
    NotStaked,

    #[msg("No tokens to stake")]
    NoToken,

    #[msg("Invalid staker")]
    InvalidStaker,

    #[msg("Invalid mint")]
    InvalidMint,
}
