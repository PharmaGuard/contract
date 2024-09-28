use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)]
pub struct InitializedManufacturer<'info> {
    #[account(
        init, 
        payer = user, 
        space = std::mem::size_of::<ManufacturerData>() + 8,
    )]
    pub manufacturer: Account<'info, ManufacturerData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialized_manufacturer_process(
    ctx: Context<InitializedManufacturer>, 
) -> Result<()> {
    msg!("Manufacturer initialized");
    Ok(())
}
