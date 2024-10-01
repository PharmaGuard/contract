use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::*;

#[event]
pub struct SendOutEvent {
    pub user_pharmacy_associated_account: Pubkey,
    pub drug: Pubkey,
    pub pharmacy_info_account: Pubkey,
}

#[derive(Accounts)]
pub struct SendOut<'info> {
    #[account(
        mut,
        seeds = [
            user_pharmacy_associated_account.authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump = user_pharmacy_associated_account.bump,
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    pub drug: Account<'info, Drug>,
    #[account(
        mut,
        seeds = [authority.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump,
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    pub authority: Signer<'info>
}

pub fn send_out_process(ctx: Context<SendOut>) -> Result<()> {
    let user_pharmacy_associated_account = &mut ctx.accounts.user_pharmacy_associated_account;
    let pharmacy_info_account_pk = &ctx.accounts.pharmacy_info_account.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user_pharmacy_associated_account.check_pharmacy_info_exists(pharmacy_info_account_pk) {
        return Err(ErrorCode::PharmacyNotAssociated.into());
    }

    let pharmacy_info =
        user_pharmacy_associated_account.get_pharmacy_account(pharmacy_info_account_pk)?;

    pharmacy_info.send_out(drug_pk)?;

    emit!(SendOutEvent {
        user_pharmacy_associated_account: user_pharmacy_associated_account.key(),
        drug: *drug_pk,
        pharmacy_info_account: *pharmacy_info_account_pk,
    });

    Ok(())
}

#[event]
pub struct LossDrugEvent {
    pub user_pharmacy_associated_account: Pubkey,
    pub drug: Pubkey,
    pub pharmacy_info_account: Pubkey,
}

#[derive(Accounts)]
pub struct LossDrug<'info> {
    #[account(
        mut,
        seeds = [
            user_pharmacy_associated_account.authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump = user_pharmacy_associated_account.bump,
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    pub drug: Account<'info, Drug>,
    #[account(
        mut,
        seeds = [authority.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump,
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn loss_drug_process(ctx: Context<LossDrug>) -> Result<()> {
    let user_pharmacy_associated_account = &mut ctx.accounts.user_pharmacy_associated_account;
    let pharmacy_info_account_pk = &ctx.accounts.pharmacy_info_account.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user_pharmacy_associated_account.check_pharmacy_info_exists(pharmacy_info_account_pk) {
        return Err(ErrorCode::PharmacyNotAssociated.into());
    }

    let pharmacy_info = user_pharmacy_associated_account.get_pharmacy_account(pharmacy_info_account_pk)?;

    pharmacy_info.loss_drug(drug_pk)?;

    emit!(LossDrugEvent {
        user_pharmacy_associated_account: user_pharmacy_associated_account.key(),
        drug: *drug_pk,
        pharmacy_info_account: *pharmacy_info_account_pk,
    });

    Ok(())
}
