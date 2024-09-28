pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use error::*;
use instructions::*;
use state::*;

declare_id!("7Yh9T122Q7guDzQe2FSYRcKj443AUvNpv3P9PeB7vGCt");

#[program]
pub mod pharma_guard_contract {
    use super::*;

    pub fn initialized_manufacturer(ctx: Context<InitializedManufacturer>) -> Result<()> {
        initialized_manufacturer_process(ctx)
    }

    pub fn add_medication(
        ctx: Context<AddMedication>,
        temperature: i8,
        expiration_date: i64,
    ) -> Result<()> {
        add_medication_process(ctx, temperature, expiration_date)
    }

    pub fn set_expired_medication(ctx: Context<ExpiredMedication>, index: u8) -> Result<()> {
        set_expired_medication_process(ctx, index.into())
    }

    pub fn reset_medication(ctx: Context<ResetMedication>, index: u8) -> Result<()> {
        reset_medication_process(ctx, index.into())
    }
}
