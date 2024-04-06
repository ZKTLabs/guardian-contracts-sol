use anchor_lang::prelude::*;

use super::proposal::Proposal;

#[account]
pub struct ComplianceRegistry {
    pub access_registry: Pubkey,
    pub whitelists: u64,
    pub blacklists: u64,
}

impl ComplianceRegistry {
    pub const SIZE: usize = core::mem::size_of::<Self>();
    
    pub(crate) fn register(&mut self, is_whitelist: bool) {
        if is_whitelist {
            self.whitelists += 1;
        } else {
            self.blacklists += 1;
        }
    }
    
    pub(crate) fn revoke(&mut self, is_whitelist: bool) {
        if is_whitelist {
            self.whitelists -= 1;
        } else {
            self.blacklists -= 1;
        }
    }
}

#[account]
pub struct Compliance {
    pub payer: Pubkey,
    pub proposal: Pubkey,
    pub speaker: Pubkey,
    pub is_whitelist: bool,
    pub description: String,
}

impl Compliance {
    pub const SIZE: usize = 32 + 32 + 32 + 1 + 256;

    pub(crate) fn new(payer: Pubkey, proposal_key: Pubkey, proposal: &Proposal) -> Self {
        Self {
            payer,
            proposal: proposal_key,
            speaker: proposal.speaker,
            description: proposal.description.clone(),
            is_whitelist: proposal.is_whitelist,
        }
    }
}