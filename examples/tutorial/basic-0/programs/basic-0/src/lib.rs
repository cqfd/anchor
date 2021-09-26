use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic_0 {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Err(CustomError::SomethingElse.into())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error]
pub enum CustomError {
    #[msg("Something went wrong!")]
    Whoops,
    #[msg("Something else went wrong!")]
    SomethingElse,
}
