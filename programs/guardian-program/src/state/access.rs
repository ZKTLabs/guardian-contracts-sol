use std::fmt::Display;
use anchor_lang::prelude::*;

#[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum Role {
    Speaker,
    Voter,
    Guardian,
    ProposalManager,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Speaker => "speaker".to_string(),
            Self::Voter => "voter".to_string(),
            Self::Guardian => "guardian".to_string(),
            Self::ProposalManager => "proposal_manager".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[account]
pub struct AccessRegistry {
    pub admin: Pubkey,
    pub bump: [u8; 1],
    pub speakers: u32,
    pub voters: u32,
    pub guardians: u32,
    pub proposal_managers: u32,
    pub _reserve: [u8; 32],
}

impl AccessRegistry {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub(crate) fn assign_role(&mut self, role: Role) {
        match role {
            Role::Speaker => { self.speakers += 1; },
            Role::Voter => { self.voters += 1; },
            Role::Guardian => { self.guardians += 1; },
            Role::ProposalManager => { self.proposal_managers += 1; },
        }
    }

    pub(crate) fn revoke_role(&mut self, role: Role) {
        match role {
            Role::Speaker => { self.speakers -= 1; },
            Role::Voter => { self.voters -= 1; },
            Role::Guardian => { self.guardians -= 1; },
            Role::ProposalManager => { self.proposal_managers -= 1; },
        }
    }
}

#[account]
pub struct Access {
    pub access_registry: Pubkey,
    pub role: Role,
    pub owner: Pubkey,
}

impl Access {
    pub const SIZE: usize = std::mem::size_of::<Self>();
}