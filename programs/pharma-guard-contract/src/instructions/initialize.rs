use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitialUser<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<User>(),
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitialUserEvent {
    user_account_pk: Pubkey,
}

pub fn initial_user_process(ctx: Context<InitialUser>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.new(ctx.accounts.authority.key())?;

    emit!(InitialUserEvent {
        user_account_pk: user.key()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitialPharmacy<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<Pharmacy>(),
    )]
    pub pharmacy_account: Account<'info, Pharmacy>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitialPharmacyEvent {
    pharmacy_account_pk: Pubkey,
}

pub fn initial_pharmacy_process(
    ctx: Context<InitialPharmacy>,
    license_number: u32,
    phone_number: u32,
) -> Result<()> {
    let pharmacy_account = &mut ctx.accounts.pharmacy_account;
    pharmacy_account.new(license_number, phone_number)?;

    emit!(InitialPharmacyEvent {
        pharmacy_account_pk: pharmacy_account.key()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitialDrug<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<Drug>(),
    )]
    pub drug_account: Account<'info, Drug>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitialDrugEvent {
    pub temperature: i8,
    pub batch_number: u32,
}

pub fn initial_drug_process(
    ctx: Context<InitialDrug>,
    temperature: i8,
    batch_number: u32,
) -> Result<()> {
    let drug_account = &mut ctx.accounts.drug_account;
    drug_account.new(temperature, batch_number)?;

    emit!(InitialDrugEvent {
        temperature: temperature,
        batch_number: batch_number
    });

    Ok(())
}
