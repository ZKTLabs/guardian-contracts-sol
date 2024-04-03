use std::fmt::Display;
use anchor_lang::prelude::*;

use crate::error::ZktGuardianError;

const MAX_GROUP_SIZE: usize = 32;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub(crate) enum Role {
    Speaker,
    Voter,
    Guardian,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Speaker => "speaker".to_string(),
            Self::Voter => "voter".to_string(),
            Self::Guardian => "guardian".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[account]
pub struct AccessGlobal {
    pub admin: Pubkey,
    pub bump: [u8; 1],
    pub speaker_groups: u64,
    pub voter_groups: u64,
    pub guardian_groups: u64,
}

impl AccessGlobal {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub(crate) fn add_group(&mut self, role: &Role, accounts: Vec<Pubkey>) -> AccessGroup {
        match role {
            Role::Speaker => { self.speaker_groups += 1; }
            Role::Voter => { self.voter_groups += 1; }
            Role::Guardian => { self.guardian_groups += 1; }
        }
        AccessGroup(accounts)
    }
}

#[account]
pub(crate) struct AccessGroup(Vec<Pubkey>);

impl AccessGroup {
    pub const SIZE: usize = MAX_GROUP_SIZE * 32;

    pub(crate) fn has_access(&self, account: &Pubkey) -> bool {
        self.0.contains(account)
    }
}