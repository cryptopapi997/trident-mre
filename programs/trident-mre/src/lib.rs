//! This program is used for MXE management and computation scheduling.

use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

declare_id!("85R8U76mZMurovUW3Houj4oeJ4W3SH6PXbJuHoMzbQDZ");

#[program]
pub mod arcium_compute {
    use super::*;

    pub fn my_ix(ctx: Context<InitPersistentMXE>) -> Result<()> {
        instructions::init_persistent_mxe(ctx, 0)
    }
}
