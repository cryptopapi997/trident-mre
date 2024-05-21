//! The Persistent MXE related instructions.
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_mxe_offset: u32)]
pub struct InitPersistentMXE<'info> {
    #[account(mut)]
    /// Signer of the transaction.
    pub signer: Signer<'info>,
    /// Authority to initialize the MXE.
    pub option: Option<AccountInfo<'info>>,
    /// System program account.
    pub system_program: Program<'info, System>,
}

/// Initializes a persistent MXE.
pub fn init_persistent_mxe(ctx: Context<InitPersistentMXE>, _mxe_offset: u32) -> Result<()> {
    Ok(())
}
