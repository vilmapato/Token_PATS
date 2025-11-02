use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{self, InitializeMint2, update_metadata, spl_token_2022::{extension::ExtensionType, pod::PodMint} }, token_interface::{MetadataPointerInitialize, token_metadata}};


pub fn initialize_mint(ctx: Context<InitializeMintSimple>)-> Result<()> {
    //define contx
    let creator = &ctx.accounts.creator;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    
    //init mint
    let mint_ctx = CpiContext::new(
        token_program.to_account_info(),
        InitializeMint2 {
            mint: mint.to_account_info(),

        },
    ); 
    let meta_ptr_ctx = CpiContext::new(
        token_program.to_account_info(),
        MetadataPointerInitialize {
            mint: mint.to_account_info(),
            token_program_id: token_program.to_account_info(),
        },
    );

    let update_meta_ctx = CpiContext::new(
        token_program.to_account_info(),
        UpdateMetadata {
            token_program.to_account_info(),
            mint: mint.to_account_info(),
        },
    );

    update_metadata(
        update_meta_ctx,
        "PATS Token".to_string(),
        "PATS".to_string(),
        "https://raw.githubusercontent.com/vilmapato/mint-token-pats/main/metadata.json".to_string(),
    )?;
    msg!("✅ Metadata successfully set for PATS Token!");
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
        space = ExtensionType::try_calculate_account_len::<PodMint>(&[ExtensionType::MetadataPointer])?,
        owner = token_program.key()
    )]
    pub mint: AccountInfo<'info>,
    pub token_program: Program<'info, token_2022::Token2022>,
    pub system_program: Program<'info, System>
}