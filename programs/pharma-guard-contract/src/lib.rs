pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7Yh9T122Q7guDzQe2FSYRcKj443AUvNpv3P9PeB7vGCt");

#[program]
pub mod pharma_guard_contract {
    use super::*;


    // initial
    pub fn initial_user_paa(ctx: Context<InitialUserPharmacyAssociatedAccount>) -> Result<()> {
        initial_user_paa_process(ctx)
    }

    pub fn initial_pharmacy(
        ctx: Context<InitialPharmacyAccountInfo>,
        license_number: u32,
        phone_number: u32,
    ) -> Result<()> {
        initial_pharmacy_process(ctx, license_number, phone_number)
    }

    pub fn initial_drug(
        ctx: Context<InitialDrug>,
        price: u64,
        temperature: i8,
        batch_number: u32,
    ) -> Result<()> {
        initial_drug_process(ctx, price, temperature, batch_number)
    }

    // user actions
    pub fn takeout_order(ctx: Context<TakeOutOrder>) -> Result<()> {
        takeout_order_process(ctx)
    }

    pub fn sign_for(ctx: Context<SignFor>) -> Result<()> {
        sign_for_process(ctx)
    }

    pub fn bind_pharmacy(ctx: Context<BindPharmacy>) -> Result<()> {
        bind_pharmacy_process(ctx)
    }

    // pharmacy actions
    pub fn send_out(ctx: Context<SendOut>) -> Result<()> {
        send_out_process(ctx)
    }

    pub fn loss_drug(ctx: Context<LossDrug>) -> Result<()> {
        loss_drug_process(ctx)
    }
}
