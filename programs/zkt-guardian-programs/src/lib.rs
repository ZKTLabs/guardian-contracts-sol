mod instructions;
mod state;
mod error;

use anchor_lang::prelude::*;

declare_id!("EFi3XHwSt8gkMP8DpMdF719YNtZYAqmazcn2gYWCWg7K");

#[program]
pub mod zkt_guardian_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
