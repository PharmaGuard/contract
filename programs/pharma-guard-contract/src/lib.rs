pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("7Yh9T122Q7guDzQe2FSYRcKj443AUvNpv3P9PeB7vGCt");

#[program]
pub mod pharma_guard_contract {
    use super::*;

    pub fn create_medication(
        ctx: Context<CreateMedication>,
        name: String,
        manufacturer: String,
        temperature: i8,
    ) -> Result<()> {
        create_medication_process(ctx, name, manufacturer, temperature)
    }

    pub fn update_medication(
        ctx: Context<UpdateMedication>,
        name: String,
        new_manufacturer: String,
        new_temperature: i8,
    ) -> Result<()> {
        update_medication_process(ctx, name, new_manufacturer, new_temperature)
    }

    pub fn delete_medication(ctx: Context<DeleteMedication>, name: String) -> Result<()> {
        delete_medication_process(ctx,name)
    }
}
