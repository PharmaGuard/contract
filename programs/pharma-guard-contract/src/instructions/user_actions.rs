use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use anchor_spl::token_2022::*;

#[event]
pub struct TakeOutEvent {
    pub user_pharmacy_associated_account: Pubkey,
    pub drug: Pubkey,
    pub pharmacy_info_account: Pubkey,
}

#[derive(Accounts)]
pub struct TakeOutOrder<'info> {
    #[account(
        mut,
        seeds = [
            authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump = user_pharmacy_associated_account.bump,
        // has_one = authority,
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    pub drug: Account<'info, Drug>,
    #[account(
        mut,
        seeds = [pharmacy_info_account.authority.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump = pharmacy_info_account.bump,
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    pub authority: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

pub fn takeout_order_process(ctx: Context<TakeOutOrder>) -> Result<()> {
    let user_pharmacy_associated_account = &mut ctx.accounts.user_pharmacy_associated_account;
    let pharmacy_info_pk = &ctx.accounts.pharmacy_info_account.key();
    let drug_pk = &ctx.accounts.drug.key();

    if !user_pharmacy_associated_account.check_pharmacy_info_exists(pharmacy_info_pk) {
        return Err(ErrorCode::PharmacyDoesNotBound.into());
    }

    let order = Order::new(drug_pk);

    user_pharmacy_associated_account.add_order(pharmacy_info_pk, order)?;

    emit!(TakeOutEvent {
        user_pharmacy_associated_account: user_pharmacy_associated_account.key(),
        drug: ctx.accounts.drug.key(),
        pharmacy_info_account: *pharmacy_info_pk,
    });

    Ok(())
}

#[event]
pub struct SignForEvent {
    pub user_pharmacy_associated_account: Pubkey,
    pub drug: Pubkey,
    pub pharmacy_info_account: Pubkey,
}

#[derive(Accounts)]
pub struct SignFor<'info> {
    #[account(
        mut,
        seeds = [
            authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump = user_pharmacy_associated_account.bump,
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    #[account(
        mut,
        seeds = [pharmacy_info_account.authority.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump,
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    #[account(
        mut,
        token::token_program = token_program
    )]
    pub user_sol_ata: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        token::token_program = token_program
    )]
    pub pharmacy_sol_ata: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub drug: Account<'info, Drug>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

pub fn sign_for_process(ctx: Context<SignFor>) -> Result<()> {
    let SignFor {
        user_pharmacy_associated_account,
        pharmacy_info_account,
        user_sol_ata,
        pharmacy_sol_ata,
        drug,
        authority,
        token_mint,
        token_program,
    } = ctx.accounts;

    if !user_pharmacy_associated_account.check_pharmacy_info_exists(&pharmacy_info_account.key()) {
        return Err(ErrorCode::PharmacyDoesNotBound.into());
    }

    let pharmacy_info = user_pharmacy_associated_account.get_pharmacy_account(
        &pharmacy_info_account.key()
    )?;

    pharmacy_info.sign_for(&drug.key());

    let cpi_program = token_program.to_account_info();
    let cpi_accounts = TransferChecked {
        from: user_sol_ata.to_account_info(),
        to: pharmacy_sol_ata.to_account_info(),
        authority: authority.to_account_info(),
        mint: token_mint.to_account_info(),
    };
    let decimals = token_mint.decimals;
    let price = drug.price;
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_context, price, decimals)?;

    emit!(SignForEvent {
        user_pharmacy_associated_account: user_pharmacy_associated_account.key(),
        drug: drug.key(),
        pharmacy_info_account: pharmacy_info_account.key(),
    });

    Ok(())
}

#[event]
pub struct BindPharmacyEvent {
    pub user_pharmacy_associated_account: Pubkey,
    pub pharmacy_info_account: Pubkey,
}

#[derive(Accounts)]
pub struct BindPharmacy<'info> {
    #[account(
        mut,
        seeds = [
            authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump = user_pharmacy_associated_account.bump,
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    #[account(
        mut,
        seeds = [pharmacy_info_account.authority.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump,
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    pub authority: Signer<'info>,
}

pub fn bind_pharmacy_process(ctx: Context<BindPharmacy>) -> Result<()> {
    let user_pharmacy_associated_account = &mut ctx.accounts.user_pharmacy_associated_account;
    let pharmacy_info_account = &mut ctx.accounts.pharmacy_info_account;

    if user_pharmacy_associated_account.check_pharmacy_info_exists(&pharmacy_info_account.key()) {
        return Err(ErrorCode::PharmacyAlreadyBound.into());
    }

    user_pharmacy_associated_account.add_pharmacy_info(&pharmacy_info_account.key());

    emit!(BindPharmacyEvent {
        user_pharmacy_associated_account: user_pharmacy_associated_account.key(),
        pharmacy_info_account: pharmacy_info_account.key(),
    });

    Ok(())
}
