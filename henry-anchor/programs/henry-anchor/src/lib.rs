use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod henry_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let acc = &mut ctx.accounts;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{}
