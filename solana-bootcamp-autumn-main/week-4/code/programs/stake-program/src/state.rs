use anchor_lang::prelude::*;

// Dữ liệu lưu trong tài khoản stake_info
#[account]
#[derive(InitSpace)] // Tự tính dung lượng cấp phát
pub struct StakeInfo {
    pub staker: Pubkey,  // người stake
    pub mint: Pubkey,    // token đã stake
    pub stake_at: u64,   // slot khi stake
      pub is_staked: bool,
    pub amount: u64,     // số lượng token đang stake
}
