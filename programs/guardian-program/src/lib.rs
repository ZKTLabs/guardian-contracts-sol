pub mod instructions;
pub mod state;
pub mod error;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("EFi3XHwSt8gkMP8DpMdF719YNtZYAqmazcn2gYWCWg7K");

use state::*;
use instructions::*;

#[program]
pub mod guardian_program {
    use super::*;

    pub fn init_access_registry(ctx: Context<InitAccessRegistry>) -> Result<()> {
        _init_access_registry(ctx)
    }

    pub fn assign_role(ctx: Context<AssignRole>, role: Role) -> Result<()> {
        _assign_role(ctx, role)
    }

    pub fn revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
        _revoke_role(ctx)
    }

    pub fn init_proposal_registry(ctx: Context<InitProposalRegistry>) -> Result<()> {
        _init_proposal_registry(ctx)
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String, is_whitelist: bool) -> Result<()> {
        _create_proposal(ctx, description, is_whitelist)
    }

    pub fn vote_for_proposal(ctx: Context<VoteForProposal>) -> Result<()> {
        _vote_for_proposal(ctx)
    }

    pub fn reject_expired_proposal(ctx: Context<RejectExpiredProposal>) -> Result<()> {
        _reject_expired_proposal(ctx)
    }

    macro_rules! impl_execute_proposal {
        ($num:tt) => {
            paste::paste! {
                pub fn [<execute_proposal_ $num>](
                    ctx: Context<[<ExecuteProposal $num>]>,
                ) -> Result<()> {
                    [<_execute_proposal_ $num>](ctx)
                }
            }
        }
    }

    impl_execute_proposal!(1);
    impl_execute_proposal!(2);
    impl_execute_proposal!(3);
    impl_execute_proposal!(4);
    impl_execute_proposal!(5);
    impl_execute_proposal!(6);
    impl_execute_proposal!(7);
    impl_execute_proposal!(8);
    impl_execute_proposal!(9);
    impl_execute_proposal!(10);
    impl_execute_proposal!(11);
    impl_execute_proposal!(12);
    impl_execute_proposal!(13);
    impl_execute_proposal!(14);
    impl_execute_proposal!(15);
    impl_execute_proposal!(16);
    impl_execute_proposal!(17);
    impl_execute_proposal!(18);
    impl_execute_proposal!(19);
    impl_execute_proposal!(20);
    
    pub fn init_compliance_registry(ctx: Context<InitComplianceRegistry>) -> Result<()> {
        _init_compliance_registry(ctx)
    }
    
    pub fn revoke_compliance(ctx: Context<RevokeCompliance>) -> Result<()> {
        _revoke_compliance(ctx)
    }
}
