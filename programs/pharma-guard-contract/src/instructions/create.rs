use anchor_lang::prelude::*;


#[account]
pub struct MedicationData {
    pub name: String,
    pub manufacturer: String,
    pub temperature: i8,
}

#[derive(Accounts)]
pub struct CreateMedication<'info> {
    #[account(
        init, 
        seeds = [b"medication", user.key().as_ref()],
        bump,
        payer = user, 
        space = 8 + std::mem::size_of::<MedicationData>(),
    )]
    pub medication: Account<'info, MedicationData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn process_create(ctx: Context<CreateMedication>, data: MedicationData) -> Result<()> {
    let medication = &mut ctx.accounts.medication;


    medication.name = data.name;
    medication.manufacturer = data.manufacturer;
    medication.temperature = data.temperature;

    Ok(())
}
