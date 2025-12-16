use anchor_lang::prelude::*;

mod instructions;
mod interfaces;
mod state;

use instructions::swap::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        swap::handler(ctx, amount)
    }
}
