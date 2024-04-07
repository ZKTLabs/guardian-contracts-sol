use anchor_lang::prelude::*;

declare_id!("3BxNMJ7VSjtVkrDLDY93D214MmQPBEx9LvTXu5gKxJkX");

#[error_code]
pub enum ErrorCode {
    #[msg("Not in whitelist")]
    NotInWhitelist,
}

#[program]
pub mod example_program {
    use super::*;

    pub fn swap(_ctx: Context<Swap>, _amount_in: u64, _min_amount_out: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    // compliance constraint
    pub owner: Signer<'info>,
    #[account(
        seeds = [
            b"compliance",
            guardian_program::utils::COMPLIANCE_REGISTRY.as_ref(),
            owner.key().as_ref(),
        ],
        bump = compliance.bump,
        seeds::program = guardian_program::id(),
        constraint = compliance.is_whitelist @ ErrorCode::NotInWhitelist,
    )]
    pub compliance: Box<Account<'info, guardian_program::state::Compliance>>,
    // swap accounts
    // ...
}