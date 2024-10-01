use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface;

#[derive(Accounts)]
pub struct InitialUserPharmacyAssociatedAccount<'info> {
    #[account(
        init,
        seeds = [
            authority.key().as_ref(),
            USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED.as_bytes(),
        ],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserPharmacyAssociatedAccount>()
    )]
    pub user_pharmacy_associated_account: Box<Account<'info, UserPharmacyAssociatedAccount>>,
    #[account(
        init,
        token::mint = token_mint,
        token::authority = authority,
        token::token_program = token_program,
        seeds = [authority.key().as_ref(), USER_ATA_SEED.as_bytes()],
        bump,
        payer = authority
    )]
    pub sol_ata: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mint::token_program = token_program, mint::authority = authority)]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

#[event]
pub struct InitialUserPAAEvent {
    user_account_pk: Pubkey,
}

pub fn initial_user_paa_process(ctx: Context<InitialUserPharmacyAssociatedAccount>) -> Result<()> {
    let user_paa = &mut ctx.accounts.user_pharmacy_associated_account;

    user_paa.new(
        ctx.accounts.authority.key(),
        ctx.bumps.user_pharmacy_associated_account,
        ctx.bumps.sol_ata
    )?;

    emit!(InitialUserPAAEvent {
        user_account_pk: user_paa.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitialPharmacyAccountInfo<'info> {
    #[account(
        init,
        seeds = [payer.key().as_ref(), PHARMACY_INFO_ACCOUNT_SEED.as_bytes()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<PharmacyInfoAccount>()
    )]
    pub pharmacy_info_account: Account<'info, PharmacyInfoAccount>,
    #[account(
        init,
        token::mint = token_mint,
        token::authority = payer,
        token::token_program = token_program,
        seeds = [payer.key().as_ref(), PHARMACY_ATA_SEED.as_bytes()],
        bump,
        payer = payer
    )]
    pub sol_ata: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

#[event]
pub struct InitialPharmacyEvent {
    pharmacy_account_pk: Pubkey,
}

pub fn initial_pharmacy_process(
    ctx: Context<InitialPharmacyAccountInfo>,
    license_number: u32,
    phone_number: u32
) -> Result<()> {
    let pharmacy_info_account = &mut ctx.accounts.pharmacy_info_account;
    pharmacy_info_account.new(
        ctx.bumps.pharmacy_info_account,
        ctx.bumps.sol_ata,
        ctx.accounts.payer.key(),
        license_number,
        phone_number
    )?;

    emit!(InitialPharmacyEvent {
        pharmacy_account_pk: pharmacy_info_account.key(),
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitialDrug<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Drug>())]
    pub drug_account: Account<'info, Drug>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitialDrugEvent {
    pub temperature: i8,
    pub batch_number: u32,
}

pub fn initial_drug_process(
    ctx: Context<InitialDrug>,
    price: u64,
    temperature: i8,
    batch_number: u32
) -> Result<()> {
    let drug_account = &mut ctx.accounts.drug_account;
    drug_account.new(price, temperature, batch_number)?;

    emit!(InitialDrugEvent {
        temperature: temperature,
        batch_number: batch_number,
    });

    Ok(())
}
