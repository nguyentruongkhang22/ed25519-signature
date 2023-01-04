use solana_program::{
  sysvar::{
    instructions::{
      ID as SYSVAR_INSTRUCTION_ID,
    },
  },
};
use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(derivation_path: Vec<u8>)]
pub struct CreateCounterContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init,
    payer = signer,
    space = 32,
    seeds = [&derivation_path],
    bump
 )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompareMessageSignatureContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateMessageSignatureContext<'info> {

  #[account(mut)]
  pub counter: Account<'info, Counter>,

  /// CHECK: Solana native Instructions SysVar
  #[account(
    constraint = instructions.key() == SYSVAR_INSTRUCTION_ID @ErrorCode::InvalidAccount,
  )]
  pub instructions: AccountInfo<'info>,
}
