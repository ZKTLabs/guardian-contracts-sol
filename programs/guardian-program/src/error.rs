use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ZktGuardianError {
    #[msg("Only pending proposal")]
    OnlyPendingProposal,
    #[msg("Proposal is not approved")]
    ProposalNotApproved,
    #[msg("Proposal is not expired")]
    ProposalNotExpired,
    #[msg("Unmatched target accounts length")]
    UnmatchedTargetAccountsLength,
    #[msg("Too many target accounts")]
    TooManyTargetAccounts,
    #[msg("Must be speaker role")]
    MustBeSpeaker,
    #[msg("Must be guardian role")]
    MustBeGuardian,
    #[msg("Must be proposal manager")]
    MustBeProposalManager,
    #[msg("Unmatched access registry")]
    UnmatchedAccessRegistry,
}