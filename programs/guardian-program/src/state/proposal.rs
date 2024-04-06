use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

use crate::error::ZktGuardianError;

const EXPIRE_DAYS: i64 = 7 * 3600 * 24;
const MAX_TARGET_ACCOUNTS: usize = 20;

#[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
}

impl std::fmt::Display for ProposalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let std = match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        };
        write!(f, "{}", std)
    }
}

#[account]
pub struct ProposalRegistry {
    pub access_registry: Pubkey,
    pub pending: u64,
    pub approved: u64,
    pub rejected: u64,
}

impl ProposalRegistry {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

#[account]
pub struct Proposal {
    pub proposal_registry: Pubkey,
    pub speaker: Pubkey,
    pub target_accounts: Vec<Pubkey>,
    pub is_whitelist: bool,
    pub description: String,
    pub timestamp: UnixTimestamp,
    pub status: ProposalStatus,
    pub voters: u32,
    pub guardians: u32,
}

impl Proposal {
    pub const SIZE: usize = 32 + 32 + MAX_TARGET_ACCOUNTS * 32 + 1 + 256 + 8 + 1 + 4 + 4;

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        registry_key: Pubkey,
        registry: &mut ProposalRegistry,
        speaker: Pubkey,
        target_accounts: Vec<Pubkey>,
        is_whitelist: bool,
        description: String,
        clock: &Clock,
        guardians: u32,
    ) -> Result<Self> {
        require_gte!(
            MAX_TARGET_ACCOUNTS,
            target_accounts.len(),
            ZktGuardianError::TooManyTargetAccounts,
        );
        registry.pending += 1;
        
        Ok(Self {
            proposal_registry: registry_key,
            speaker,
            target_accounts,
            is_whitelist,
            description,
            timestamp: clock.unix_timestamp,
            status: ProposalStatus::Pending,
            voters: 0,
            guardians,
        })
    }

    pub(crate) fn vote(&mut self, registry: &mut ProposalRegistry) -> Result<()> {
        require_eq!(self.status, ProposalStatus::Pending, ZktGuardianError::OnlyPendingProposal);

        self.voters += 1;
        if self.voters * 2 > self.guardians {
            self.status = ProposalStatus::Approved;
            registry.pending -= 1;
            registry.approved += 1;
        }
        
        Ok(())
    }

    pub(crate) fn reject_expired(
        &mut self,
        clock: &Clock,
        registry: &mut ProposalRegistry,
    ) -> Result<()> {
        require_eq!(self.status, ProposalStatus::Pending, ZktGuardianError::OnlyPendingProposal);
        require_gte!(
            self.timestamp + EXPIRE_DAYS,
            clock.unix_timestamp,
            ZktGuardianError::ProposalNotExpired,
        );
        registry.pending -= 1;
        registry.rejected += 1;
        
        Ok(())
    }
}