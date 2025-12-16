use anchor_lang::prelude::*;
use anchor_spl::prelude::*;

pub trait TokenTransfer<'info> {
    fn transfer(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        signer_seeds: Option<&[&[u8]]>,
        amount: u64,
    ) -> Result<()>;
}

pub struct SplTokenTransfer<'info> {
    pub token_program: AccountInfo<'info>,
}

impl<'info> TokenTransfer<'info> for SplTokenTransfer<'info> {
    fn transfer(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        signer_seeds: Option<&[&[u8]]>,
        amount: u64,
    ) -> Result<()> {
        let account = Transfer {
            from,
            to,
            authority,
        };

        let ctx = match signer_seeds {
            Some(seeds) => {
                cpiContext::new_with_signer(self.token_program.clone(), account, &[seeds])
            }

            None => cpiContext::new(self.token_program.clone(), account),
        };
        transfer(ctx, amount);
    }
}
