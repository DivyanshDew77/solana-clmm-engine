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
