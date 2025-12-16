use anchor_lang::prelude::*;
use crate::state::pool::Pool;
use crate::state::vault::token_program::*;

#[derive(Accounts)]
pub struct Swap<'info>{
     
     #[account(mut)]
     pub user_token_a : AccountInfo<'info>,

     #[account(mut)]
     pub user_token_b : AccountInfo<'info>,

     #[account(mut)]
     pub vault_token_a : AccountInfo<'info>,

     #[account(mut)]
     pub vault_token_b : AccountInfo<'info>,

     #[account(
        init,
        space=Pool::INIT_SPACE,
        seeds=[b"swap"],
        bump = pool.bump,
     )]
     pub pool: Account<'info,Pool>,

    #[account(mut)]
    pub user: Signer<'info>,

    
    pub token_program: Program<'info,Token>,

}

pub fn handle(ctx:Context<Swap>,amount:u64) -> Result<()>{
let transfer = SplTokenTransfer {
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    // User → Vault
    transfer.transfer(
        ctx.accounts.user_token_a.to_account_info(),
        ctx.accounts.vault_token_a.to_account_info(),
        ctx.accounts.user.to_account_info(),
        None,
        amount,
    )?;

    // Vault → User (PDA signs)
    let seeds = &[b\"pool\", &[ctx.accounts.pool.bump]];

 

    Ok(());

}