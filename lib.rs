use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Burn, Transfer};
use std::collections::HashMap;

declare_id!("YourProgramAddressHere");

#[program]
pub mod solana_meme_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_bump: u8) -> Result<()> {
        let meme_coin = &mut ctx.accounts.meme_coin_mint;
        meme_coin.mint_authority = Some(ctx.accounts.authority.key());
        meme_coin.freeze_authority = Some(ctx.accounts.authority.key());
        Ok(())
    }

    pub fn mint_to_holders(ctx: Context<MintToHolders>, amount: u64) -> Result<()> {
        let accounts = &ctx.accounts.holder_accounts;
        for acc in accounts.iter() {
            token::mint_to(
                ctx.accounts.into_mint_to_context(),
                amount,
            )?;
        }
        Ok(())
    }

    pub fn early_sale_tax_transfer(ctx: Context<EarlySaleTaxTransfer>, amount: u64) -> Result<()> {
        let timestamp = Clock::get().unwrap().unix_timestamp;
        let holding_period = 2592000; // 30 days in seconds

        if timestamp - ctx.accounts.from.last_transaction < holding_period {
            let tax = (amount as f64 * 0.1) as u64; // 10% tax for selling early
            let after_tax_amount = amount - tax;
            **ctx.accounts.from.to_account_info().try_borrow_mut_lamports()? -= tax;
            **ctx.accounts.to.to_account_info().try_borrow_mut_lamports()? += tax;
            msg!("10% tax applied for early selling.");
            token::transfer(ctx.accounts.into_transfer_to_context(), after_tax_amount)?;
        } else {
            token::transfer(ctx.accounts.into_transfer_to_context(), amount)?;
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
pub struct MintToHolders<'info> {
    #[account(mut)]
    pub meme_coin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub holder_accounts: Vec<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EarlySaleTaxTransfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
