use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("6Uejn3594k4dQqJu1i977RAxQwXU64PhGKwKdpJffEn7");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn init_mint(_ctx: Context<InitMint>) -> ProgramResult {
        Ok(())
    }

    pub fn mint_some_tokens(ctx: Context<MintSomeTokens>, bump: u8) -> ProgramResult {
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.us.to_account_info(),
                },
                &[&[&[bump]]],
            ),
            1,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct InitMint<'info> {
    #[account(init, payer = user, mint::decimals = 6, mint::authority = us)]
    pub mint: Account<'info, Mint>,

    pub user: Signer<'info>,

    #[account(seeds = [], bump)]
    pub us: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintSomeTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = user, associated_token::mint = mint, associated_token::authority = user)]
    pub destination: Account<'info, TokenAccount>,
    pub user: Signer<'info>,

    #[account(seeds = [], bump)]
    pub us: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
