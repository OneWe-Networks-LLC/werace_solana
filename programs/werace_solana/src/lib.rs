use anchor_lang::prelude::*;

declare_id!("37qsC87eKctWhCXLRFwYmmw7zETN18mAPZuPvuuxGPzE");

#[program]
pub mod werace_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
