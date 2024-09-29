use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[event]
pub struct SendOutEvent {
    pub user: Pubkey,
    pub drug: Pubkey,
    pub pharmacy: Pubkey,
}

#[derive(Accounts)]
pub struct SendOut<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user: Account<'info, User>,
    pub drug: Account<'info, Drug>,
    pub pharmacy: Account<'info, Pharmacy>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn send_out_process(ctx: Context<SendOut>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pharmacy_pk = &ctx.accounts.pharmacy.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user.check_pharmacy_account_exists(pharmacy_pk) {
        return Err(ErrorCode::PharmacyAccountDoesNotExist.into());
    }

    let pharmacy_op = user.get_pharmacy_account(pharmacy_pk);

    if let Some(pharmacy) = pharmacy_op {
        pharmacy.send_out(drug_pk)?;
    }

    emit!(SendOutEvent {
        user: ctx.accounts.user.key(),
        drug: ctx.accounts.drug.key(),
        pharmacy: ctx.accounts.pharmacy.key(),
    });

    Ok(())
}

#[event]
pub struct LossDrugEvent {
    pub user: Pubkey,
    pub drug: Pubkey,
    pub pharmacy: Pubkey,
}

#[derive(Accounts)]
pub struct LossDrug<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user: Account<'info, User>,
    pub drug: Account<'info, Drug>,
    pub pharmacy: Account<'info, Pharmacy>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn loss_drug_process(ctx: Context<LossDrug>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pharmacy_pk = &ctx.accounts.pharmacy.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user.check_pharmacy_account_exists(pharmacy_pk) {
        return Err(ErrorCode::PharmacyAccountDoesNotExist.into());
    }

    let pharmacy_op = user.get_pharmacy_account(pharmacy_pk);

    if let Some(pharmacy) = pharmacy_op {
        pharmacy.loss_drug(drug_pk)?;
    }

    emit!(LossDrugEvent {
        user: ctx.accounts.user.key(),
        drug: ctx.accounts.drug.key(),
        pharmacy: ctx.accounts.pharmacy.key(),
    });

    Ok(())
}
