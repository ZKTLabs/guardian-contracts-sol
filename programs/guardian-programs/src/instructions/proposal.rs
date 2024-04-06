use anchor_lang::{prelude::*, Key};

use crate::{
    state::{
        AccessRegistry, Access, Role,
        ProposalRegistry, Proposal, ProposalStatus,
        ComplianceRegistry, Compliance,
    },
    error::ZktGuardianError,
};

#[derive(Accounts)]
pub struct InitProposalRegistry<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: no need to be checked
    pub access_registry: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + ProposalRegistry::SIZE,
        seeds = [b"proposal_registry", access_registry.key().as_ref()],
        bump,
    )]
    pub proposal_registry: Account<'info, ProposalRegistry>,
    // system program
    pub system_program: Program<'info, System>,
}

pub(crate) fn _init_proposal_registry(ctx: Context<InitProposalRegistry>) -> Result<()> {
    ctx.accounts.proposal_registry.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.proposal_registry.pending = 0;
    ctx.accounts.proposal_registry.approved = 0;
    ctx.accounts.proposal_registry.rejected = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(description: String, is_whitelist: bool)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub speaker: Signer<'info>,
    pub access_registry: Account<'info, AccessRegistry>,
    #[account(
        has_one = access_registry,
        constraint = access.owner == speaker.key(),
        constraint = access.role == Role::Speaker @ ZktGuardianError::MustBeSpeaker,
    )]
    pub access: Account<'info, Access>,
    #[account(mut, has_one = access_registry)]
    pub proposal_registry: Account<'info, ProposalRegistry>,
    #[account(
        init,
        payer = speaker,
        space = 8 + Proposal::SIZE,
    )]
    pub proposal: Box<Account<'info, Proposal>>,
    // system program
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn _create_proposal(
    ctx: Context<CreateProposal>,
    description: String,
    is_whitelist: bool,
) -> Result<()> {
    let proposal_registry_key = ctx.accounts.proposal_registry.key();
    let proposal = Proposal::new(
        proposal_registry_key,
        &mut ctx.accounts.proposal_registry,
        ctx.accounts.speaker.key(),
        ctx.remaining_accounts.iter().map(Key::key).collect(),
        is_whitelist,
        description,
        &ctx.accounts.clock,
        ctx.accounts.access_registry.guardians,
    )?;
    ctx.accounts.proposal.set_inner(proposal);

    Ok(())
}

#[derive(Accounts)]
pub struct VoteForProposal<'info> {
    pub guardian: Signer<'info>,
    pub access_registry: Account<'info, AccessRegistry>,
    #[account(
        has_one = access_registry,
        constraint = access.owner == guardian.key(),
        constraint = access.role == Role::Guardian @ ZktGuardianError::MustBeGuardian,
    )]
    access: Account<'info, Access>,
    #[account(mut, has_one = access_registry)]
    pub proposal_registry: Account<'info, ProposalRegistry>,
    #[account(mut, has_one = proposal_registry)]
    pub proposal: Box<Account<'info, Proposal>>,
}

pub(crate) fn _vote_for_proposal(ctx: Context<VoteForProposal>) -> Result<()> {
    ctx.accounts.proposal.vote(&mut ctx.accounts.proposal_registry)
}

#[derive(Accounts)]
pub struct RejectExpiredProposal<'info> {
    /// CHECK: no need to be checked
    pub speaker: UncheckedAccount<'info>,
    #[account(mut)]
    pub proposal_registry: Account<'info, ProposalRegistry>,
    #[account(mut, close = speaker, has_one = proposal_registry, has_one = speaker)]
    pub proposal: Box<Account<'info, Proposal>>,
    // system programs
    pub clock: Sysvar<'info, Clock>,
}

pub(crate) fn _reject_expired_proposal(ctx: Context<RejectExpiredProposal>) -> Result<()> {
    ctx.accounts.proposal.reject_expired(
        &ctx.accounts.clock,
        &mut ctx.accounts.proposal_registry,
    )
}

macro_rules! impl_execute_proposal {
    ($num:tt, [$($idx:tt),+]) => {
        paste::paste! {
            #[derive(Accounts)]
            pub struct [<ExecuteProposal $num>]<'info> {
                #[account(mut)]
                pub payer: Signer<'info>,
                /// CHECK: no need to be checked
                pub speaker: UncheckedAccount<'info>,
                pub proposal_registry: Box<Account<'info, ProposalRegistry>>,
                #[account(
                    mut,
                    close = speaker,
                    has_one = proposal_registry,
                    has_one = speaker,
                    constraint = proposal.status == ProposalStatus::Approved @ ZktGuardianError::ProposalNotApproved,
                    constraint = proposal.target_accounts.len() == $num @ ZktGuardianError::UnmatchedTargetAccountsLength,
                )]
                pub proposal: Box<Account<'info, Proposal>>,
                #[account(
                    mut,
                    constraint = compliance_registry.access_registry == proposal_registry.access_registry @ ZktGuardianError::UnmatchedAccessRegistry,
                )]
                pub compliance_registry: Box<Account<'info, ComplianceRegistry>>,
                $(
                    #[account(
                        init_if_needed,
                        payer = payer,
                        space = 8 + Compliance::SIZE,
                        seeds = [
                            b"compliance",
                            compliance_registry.key().as_ref(),
                            proposal.target_accounts[$idx].as_ref(),
                        ],
                        bump,
                    )]
                    pub [<compliance_ $idx>]: Box<Account<'info, Compliance>>,
                )+
                // system program
                pub system_program: Program<'info, System>,
            }

            pub(crate) fn [<_execute_proposal_ $num>](
                ctx: Context<[<ExecuteProposal $num>]>,
            ) -> Result<()> {
                ctx.accounts.compliance_registry.register(ctx.accounts.proposal.is_whitelist);

                let compliance = Compliance::new(
                    ctx.accounts.payer.key(),
                    ctx.accounts.proposal.key(),
                    &ctx.accounts.proposal,
                );
                $(
                    ctx.accounts.[<compliance_ $idx>].set_inner(compliance.clone());
                )+

                Ok(())
            }
        }
    };
}

impl_execute_proposal!(1, [0]);
impl_execute_proposal!(2, [0, 1]);
impl_execute_proposal!(3, [0, 1, 2]);
impl_execute_proposal!(4, [0, 1, 2, 3]);
impl_execute_proposal!(5, [0, 1, 2, 3, 4]);
impl_execute_proposal!(6, [0, 1, 2, 3, 4, 5]);
impl_execute_proposal!(7, [0, 1, 2, 3, 4, 5, 6]);
impl_execute_proposal!(8, [0, 1, 2, 3, 4, 5, 6, 7]);
impl_execute_proposal!(9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
impl_execute_proposal!(10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
impl_execute_proposal!(11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
impl_execute_proposal!(12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
impl_execute_proposal!(13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
impl_execute_proposal!(14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
impl_execute_proposal!(15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
impl_execute_proposal!(16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
impl_execute_proposal!(17, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
impl_execute_proposal!(18, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
impl_execute_proposal!(19, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
impl_execute_proposal!(20, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);