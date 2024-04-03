use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ZktGuardianError {
    #[msg("Only pending proposal")]
    OnlyPendingProposal,
    #[msg("Proposal is expired")]
    ProposalExpired,
}