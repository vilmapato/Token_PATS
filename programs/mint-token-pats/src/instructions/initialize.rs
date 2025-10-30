use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, initialize_mint2, InitializeMint2, Token2022};


pub fn initialize_mint(ctx: Context<InitializeMintSimple>)-> Result<()> {
    
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        InitializeMint2 {
            mint: ctx.accounts.mint.to_account_info(),

        },
    );
    initialize_mint2(cpi_ctx, 6, &ctx.accounts.creator.key(), None,)?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMintSimple<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: The mint account is newly created in this instruction,
    /// and owned by the Token-2022 program, so no additional checks are needed.
 
    #[account(
        init,
        payer = creator,
        space = 82, //82 bytes is the required space for a mint with InitializeMint2
        owner = token_program.key()
    )]
    pub mint: AccountInfo<'info>,
    pub token_program: Program<'info, token_2022::Token2022>,
    pub system_program: Program<'info, System>
}