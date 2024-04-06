use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, transfer};

declare_id!("3BxNMJ7VSjtVkrDLDY93D214MmQPBEx9LvTXu5gKxJkX");

#[error_code]
pub enum ErrorCode {
    #[msg("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance,
    #[msg("Pool Mint Amount < 0 on LP Deposit")]
    NoPoolMintOutput,
    #[msg("Trying to burn too much")]
    BurnTooMuch,
    #[msg("Not enough out")]
    NotEnoughOut,
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
        constraint = compliance.is_whitelist,
    )]
    pub compliance: Box<Account<'info, guardian_program::state::Compliance>>,
    // swap accounts
    pub pool_state: Box<Account<'info, PoolState>>,
    /// CHECK: this is the authority for the pool
    #[account(seeds=[b"authority", pool_state.key().as_ref()], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    #[account(mut, has_one = owner)]
    pub user_in: Box<Account<'info, TokenAccount>>,
    #[account(mut, has_one = owner)]
    pub user_out: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_in.mint == user_in.mint,
        constraint = vault_in.owner == pool_authority.key(),
    )]
    pub vault_in: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_out.mint == user_out.mint,
        constraint = vault_out.owner == pool_authority.key(),
    )]
    pub vault_out: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

// a pool without fee
#[account]
pub struct PoolState {
    pub mint0: Pubkey,
    pub mint1: Pubkey,
    pub total_amount_minted: u64,
}