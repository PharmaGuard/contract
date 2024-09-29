pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7Yh9T122Q7guDzQe2FSYRcKj443AUvNpv3P9PeB7vGCt");

#[program]
pub mod pharma_guard_contract {
    use super::*;

    pub fn initialized_user(ctx: Context<InitialUser>) -> Result<()> {
        initial_user_process(ctx)
    }

    pub fn initial_pharmacy(
        ctx: Context<InitialPharmacy>,
        license_number: u32,
        phone_number: u32,
    ) -> Result<()> {
        initial_pharmacy_process(ctx, license_number, phone_number)
    }

    pub fn initial_drug(
        ctx: Context<InitialDrug>,
        temperature: i8,
        batch_number: u32,
    ) -> Result<()> {
        initial_drug_process(ctx, temperature, batch_number)
    }

    pub fn create_order(ctx: Context<CreateOrder>) -> Result<()> {
        create_order_process(ctx)
    }

    pub fn sign_for(ctx: Context<SignFor>) -> Result<()> {
        sign_for_process(ctx)
    }

    pub fn send_out(ctx: Context<SendOut>) -> Result<()> {
        send_out_process(ctx)
    }

    pub fn loss_drug(ctx: Context<LossDrug>) -> Result<()> {
        loss_drug_process(ctx)
    }
}
