use anchor_lang::prelude::*;


#[account]
pub struct WhirlpoolsConfig {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,

    pub default_protocol_fee_rate: u16,
    pub feature_flags: u16,
}

impl WhirlpoolsConfig {
    pub const LEN: usize = 8 + 96 + 4;
}