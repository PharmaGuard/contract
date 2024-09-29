use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[event]
pub struct CreateOrderEvent {
    pub user: Pubkey,
    pub drug: Pubkey,
    pub pharmacy: Pubkey,
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub drug: Account<'info, Drug>,
    pub pharmacy: Account<'info, Pharmacy>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn create_order_process(ctx: Context<CreateOrder>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pharmacy_pk = &ctx.accounts.pharmacy.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user.check_pharmacy_account_exists(pharmacy_pk) {
        user.add_pharmacy_account(pharmacy_pk)?;
    }

    let order = Order::new(drug_pk);

    user.add_order(pharmacy_pk, order)?;

    emit!(CreateOrderEvent {
        user: ctx.accounts.user.key(),
        drug: ctx.accounts.drug.key(),
        pharmacy: ctx.accounts.pharmacy.key(),
    });

    Ok(())
}

#[event]
pub struct SignForEvent {
    pub user: Pubkey,
    pub drug: Pubkey,
    pub pharmacy: Pubkey,
}

#[derive(Accounts)]
pub struct SignFor<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user: Account<'info, User>,
    pub pharmacy: Account<'info, Pharmacy>,
    pub drug: Account<'info, Drug>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn sign_for_process(ctx: Context<SignFor>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pharmacy_pk = &ctx.accounts.pharmacy.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user.check_pharmacy_account_exists(pharmacy_pk) {
        return Err(ErrorCode::PharmacyAccountDoesNotExist.into());
    }

    let pharmacy_op = user.get_pharmacy_account(pharmacy_pk);

    if let Some(pharmacy) = pharmacy_op {
        pharmacy.sign_for(drug_pk)?;
    } else {
        return Err(ErrorCode::PharmacyAccountDoesNotExist.into());
    }

    emit!(SignForEvent {
        user: ctx.accounts.user.key(),
        drug: ctx.accounts.drug.key(),
        pharmacy: ctx.accounts.pharmacy.key(),
    });

    Ok(())
}
