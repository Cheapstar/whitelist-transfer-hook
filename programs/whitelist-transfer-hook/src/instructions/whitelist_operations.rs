use anchor_lang::{
    prelude::*, 
    system_program
};

use crate::state::whitelist::Whitelist;

#[derive(Accounts)]
#[instruction(user:Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(
        mut
    )]
    pub admin: Signer<'info>,   // this is to me seems like security vulnerability anybody can impersonate admin
    #[account(
        init,
        payer = admin,
        space = 32 + 8 + 1,
        seeds = [b"whitelisted",user.as_ref()],
        bump,
    )]
    pub whitelisted_account: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user:Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut,
        //user = 
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        close = admin,
        has_one = admin,
        seeds = [b"whitelisted",user.as_ref()],
        bump,
    )]
    pub whitelisted_account: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}



impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(&mut self) -> Result<()> {
                self.whitelisted_account.set_inner(
            Whitelist { admin: *self.admin.key, bump: self.whitelisted_account.bump });
        msg!("User {} successfully whitelisted");
        Ok(())
    }
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {

        msg!("User {} successfully whitelisted");
        Ok(())
    }
}