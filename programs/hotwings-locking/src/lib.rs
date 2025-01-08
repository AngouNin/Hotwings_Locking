use anchor_lang::prelude::*;

declare_id!("FuML3MpeXtoKgZY1nBJUCJyvtQZdBcSt2Kb7GjqGW8SR");

#[program]
pub mod hotwings_locking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
