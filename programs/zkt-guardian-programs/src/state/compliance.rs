use anchor_lang::prelude::*;

use super::proposal::Proposal;

#[account]
pub struct Compliance {
    pub proposal: Pubkey,
    pub author: Pubkey,
    pub description: String,
    pub inlist: bool,
}

impl Compliance {
    pub(crate) fn new(proposal_key: Pubkey, proposal: &Proposal) -> Self {
        Self {
            proposal: proposal_key,
            author: proposal.author,
            description: proposal.description.clone(),
            inlist: true,
        }
    }
}