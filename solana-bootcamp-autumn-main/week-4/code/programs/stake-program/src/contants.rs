use anchor_lang::constant;

// Seed để tạo PDA cho reward_vault
#[constant]
pub const REWARD_VAULT_SEED: &[u8] = b"reward";

// Seed để tạo PDA cho stake_info
#[constant]
pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
