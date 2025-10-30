use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

declare_id!("BHUveTjk2eXEZr6VQ7uEyHUHYZCb3fdJ3Ztz2PPqBwpM");

#[program]
pub mod mint_token_pats {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMintSimple>) -> Result<()> {
        instructions::initialize_mint(ctx)
    }
}


