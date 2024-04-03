use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

use crate::error::ZktGuardianError;

const EXPIRY_DAYS: i64 = 7 * 3600 * 24;

#[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
}

#[account]
pub struct Proposal {
    pub author: Pubkey,
    pub target_addresses: Vec<Pubkey>,
    pub is_whitelist: bool,
    pub description: String,
    pub timestamp: UnixTimestamp,
    pub status: ProposalStatus,
    pub voters: u64,
    pub active_nodes: u64,
}

impl Proposal {
    pub(crate) fn new(
        author: Pubkey,
        target_addresses: Vec<Pubkey>,
        is_whitelist: bool,
        description: String,
        clock: &Clock,
        active_nodes: u64,
    ) -> Self {
        Self {
            author,
            target_addresses,
            is_whitelist,
            description,
            timestamp: clock.unix_timestamp,
            status: ProposalStatus::Pending,
            voters: 0,
            active_nodes,
        }
    }

    pub(crate) fn vote(&mut self) -> Result<bool> {
        if self.status != ProposalStatus::Pending {
            return Err(ZktGuardianError::OnlyPendingProposal.into());
        }

        self.voters += 1;
        if self.voters * 2 > self.active_nodes {
            self.status = ProposalStatus::Approved;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn reject_expired(&mut self, clock: &Clock) -> Result<()> {
        if self.status != ProposalStatus::Pending {
            return Err(ZktGuardianError::OnlyPendingProposal.into());
        }

        if self.timestamp + EXPIRY_DAYS >= clock.unix_timestamp {
            Err(ZktGuardianError::ProposalExpired.into())
        } else {
            Ok(())
        }
    }
}