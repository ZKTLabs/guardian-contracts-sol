use anchor_lang::prelude::*;

use crate::{
    state::{Access, AccessRegistry, Role, ComplianceRegistry, Compliance},
    error::ZktGuardianError,
};

#[derive(Accounts)]
pub struct InitComplianceRegistry<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: no need to be checked
    pub access_registry: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + ComplianceRegistry::SIZE,
        seeds = [b"compliance_registry", access_registry.key().as_ref()],
        bump,
    )]
    pub compliance_registry: Account<'info, ComplianceRegistry>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_compliance_registry(ctx: Context<InitComplianceRegistry>) -> Result<()> {
    ctx.accounts.compliance_registry.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.compliance_registry.whitelists = 0;
    ctx.accounts.compliance_registry.blacklists = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct RevokeCompliance<'info> {
    pub compliance_manager: Signer<'info>,
    /// CHECK: no need to be checked
    pub payer: UncheckedAccount<'info>,
    /// CHECK: no need to be checked
    pub target_account: UncheckedAccount<'info>,
    pub access_registry: Account<'info, AccessRegistry>,
    #[account(
        has_one = access_registry,
        constraint = access.owner == compliance_manager.key(),
        constraint = access.role == Role::ProposalManager @ ZktGuardianError::MustBeProposalManager,
    )]
    pub access: Account<'info, Access>,
    #[account(mut, has_one = access_registry)]
    pub compliance_registry: Account<'info, ComplianceRegistry>,
    #[account(
        mut,
        close = payer,
        has_one = payer,
        seeds = [
            b"compliance",
            compliance_registry.key().as_ref(),
            target_account.key().as_ref(),
        ],
        bump = compliance.bump,
    )]
    pub compliance: Account<'info, Compliance>,
}

pub(crate) fn _revoke_compliance(ctx: Context<RevokeCompliance>) -> Result<()> {
    ctx.accounts.compliance_registry.revoke(ctx.accounts.compliance.is_whitelist);

    Ok(())
}