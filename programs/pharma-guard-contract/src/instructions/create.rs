use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String, manufacturer: String)]
pub struct CreateMedication<'info> {
    #[account(
        init, 
        seeds = [name.as_bytes(), user.key().as_ref()],
        bump,
        payer = user, 
        space = MedicationData::INIT_SPACE + name.len() + manufacturer.len(),
    )]
    pub medication: Account<'info, MedicationData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_medication_process(
    ctx: Context<CreateMedication>, 
    name: String,
    manufacturer:String,
    temperature: i8,
) -> Result<()> {
    let medication = &mut ctx.accounts.medication;


    medication.name = name;
    medication.manufacturer = manufacturer;
    medication.temperature = temperature;

    msg!("Medication created: {}", medication.name);

    Ok(())
}
