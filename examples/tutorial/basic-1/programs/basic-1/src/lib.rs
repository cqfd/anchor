use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[test]
fn size() {
    eprintln!("{}", std::mem::size_of::<Pubkey>());
}

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

    pub fn init_mint(ctx: Context<InitMint>, _mint_authority_bump: u8) -> ProgramResult {
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, mint_authority_bump: u8) -> ProgramResult {
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[b"mint_authority", &[mint_authority_bump]]],
            ),
            1,
        )
    }

    pub fn burn(ctx: Context<Burn>) -> ProgramResult {
        anchor_spl::token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.source.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            1,
        )
    }

    pub fn zero_copy(ctx: Context<ZeroCopy>) -> ProgramResult {
        let mut zero_copy = ctx.accounts.zero_copy.load_init()?;
        msg!("key = {:?}", ctx.accounts.system_program.key().to_bytes());
        zero_copy.pk = ctx.accounts.system_program.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ZeroCopy<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub zero_copy: AccountLoader<'info, Thing>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account(zero_copy)]
pub struct Thing {
    pk: Pubkey,
}

#[derive(Accounts)]
#[instruction(mint_authority_bump: u8)]
pub struct InitMint<'info> {
    #[account(init, payer = user, mint::decimals = 0, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,

    #[account(seeds = [b"mint_authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(mint_authority_bump: u8)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(seeds = [b"mint_authority".as_ref()], bump = mint_authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    pub user: Signer<'info>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = user)]
    destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub user: Signer<'info>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = user)]
    source: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
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

#[account]
pub struct MyAccount {
    pub data: u64,
}
