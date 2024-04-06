use anchor_lang::prelude::*;

use crate::state::{AccessRegistry, Access, Role};

#[derive(Accounts)]
pub struct InitAccessRegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + AccessRegistry::SIZE,
        seeds = [b"access", admin.key().as_ref()],
        bump,
    )]
    pub access_global: Account<'info, AccessRegistry>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_access_registry(ctx: Context<InitAccessRegistry>) -> Result<()> {
    ctx.accounts.access_global.admin = ctx.accounts.admin.key();
    ctx.accounts.access_global.bump = [ctx.bumps.access_global];

    Ok(())
}

#[derive(Accounts)]
#[instruction(role: Role)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: no need to be checked
    pub owner: UncheckedAccount<'info>,
    #[account(mut, has_one = admin)]
    pub access_registry: Account<'info, AccessRegistry>,
    #[account(
        init,
        payer = admin,
        space = 8 + Access::SIZE,
        seeds = [
            b"access",
            access_registry.key().as_ref(),
            role.to_string().as_bytes(),
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub access: Account<'info, Access>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _assign_role(ctx: Context<AssignRole>, role: Role) -> Result<()> {
    ctx.accounts.access_registry.assign_role(role);
    
    ctx.accounts.access.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.access.role = role;
    ctx.accounts.access.owner = ctx.accounts.owner.key();

    Ok(())
}

#[derive(Accounts)]
pub struct RevokeRole<'info> {
    pub admin: Signer<'info>,
    #[account(mut, has_one = admin)]
    pub access_registry: Account<'info, AccessRegistry>,
    #[account(mut, close = admin)]
    pub access: Account<'info, Access>,
}

pub(crate) fn _revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
    ctx.accounts.access_registry.revoke_role(ctx.accounts.access.role);
    
    Ok(())
}
