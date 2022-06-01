use anchor_lang::prelude::*;

declare_id!("HRXo48WtYjQjoGnNF7tbSsMtPVHqCzkLoBV6AoysG14V");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
