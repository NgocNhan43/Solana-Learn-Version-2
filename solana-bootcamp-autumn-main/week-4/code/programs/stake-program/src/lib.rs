// Khai báo thư viện Anchor
use anchor_lang::prelude::*;
use instructions::*; // Import các hàm từ module instructions

// Khai báo các module con
mod contants;
mod errors;
mod instructions;
mod state;
// ID chương trình (cần khớp với ID deploy trên Solana)
declare_id!("5ZH5NAc5AeWpYW5MgxDgsHSPjzBYmN6qbn1dSwTYBj6X");

#[program]
pub mod stake_program {
    use super::*;

    // Khởi tạo reward vault
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    // Stake token
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    // Unstake token + nhận phần thưởng (hỗ trợ unstake một phần)
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake(ctx, amount)
    }
}
