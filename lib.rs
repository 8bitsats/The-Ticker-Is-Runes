use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFznzQbYbMYhHus4DxKWwo6mHEPSEPvFqinqRZbg");

#[program]
pub mod solana_meme_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_bump: u8) -> Result<()> {
        ctx.accounts.meme_coin_mint.mint_authority = ctx.accounts.authority.key();
        ctx.accounts.meme_coin_mint.freeze_authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn airdrop_tokens(ctx: Context<Airdrop>, amount: u64) -> Result<()> {
        let airdrop_accounts = &ctx.accounts.holder_accounts;
        for acc in airdrop_accounts.iter() {
            token::mint_to(
                ctx.accounts.into_mint_to_context(),
                amount,
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 9000, seeds = [b"meme_coin_mint"], bump)]
    pub meme_coin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub meme_coin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub holder_accounts: Vec<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}
