use anchor_lang::prelude::*;

declare_id!("Ddy2wHW8ZGgtVgPCoriZs9FtcsxkbwLTT2Gxzjyg1KPq");

#[program]
pub mod solana_clmm_engine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
#[derive(Default)]
pub struct Whirlpool {
    pub whirlpools_config: Pubkey, //Controller Account of all the pools

    pub whirlpool_bump: [u8; 1],

    pub fee_teir_index_seed: [u8;2],

    pub tick_spacing: u16,

    pub fee_rate: u16,

    pub protocol_fee_rate: u16,

    pub liquidity: u128,

    pub sqrt_price: u128,

    pub tick_current_index: i32,

    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,

    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,

    pub fee_growth_global_a: u128,

    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,

    pub fee_growth_global_b: u128,

    pub reward_last_updated_timestamp: u64,

}