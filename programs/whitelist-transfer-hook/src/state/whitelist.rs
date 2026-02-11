use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub admin: Pubkey,       // not scalable
    pub bump: u8,
}