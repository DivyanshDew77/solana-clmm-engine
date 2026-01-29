use crate::state::WhirlpoolsConfig;
use anchor_lang::prelude::*;

#[account]
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u18
}