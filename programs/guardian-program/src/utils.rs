use anchor_lang::prelude::*;
use solana_program::pubkey;

type Compliance<'info> = Account<'info, crate::state::Compliance>;

pub const COMPLIANCE_REGISTRY: Pubkey = pubkey!("EFi3XHwSt8gkMP8DpMdF719YNtZYAqmazcn2gYWCWg7K");

pub fn is_whitelist<'info>(account_info: &'info AccountInfo<'info>) -> bool {
    if let Ok(compliance) = Compliance::try_from(account_info) {
        compliance.is_whitelist
    } else {
        false
    }
}

pub fn is_blacklist<'info>(account_info: &'info AccountInfo<'info>) -> bool {
    if let Ok(compliance) = Compliance::try_from(account_info) {
        !compliance.is_whitelist
    } else {
        false
    }
}