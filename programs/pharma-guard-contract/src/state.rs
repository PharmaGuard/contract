use crate::error::ErrorCode;
use anchor_lang::prelude::*;

pub const INITIAL: u8 = 1 << 0;
pub const IN_DELIVERY: u8 = 1 << 1;
pub const CANCELLED: u8 = 1 << 2;
pub const COMPLETED: u8 = 1 << 3;

pub const MEDICATION_NORMAL: u8 = 1 << 4;
pub const MEDICATION_LOST: u8 = 1 << 5;

const MAX_ORDERS: usize = 2;
const MAX_PHARMACIES: usize = 2;

pub const USER_PHARMACY_ASSOCIATED_ACCOUNT_SEED: &str = "user_pharmacy_associated_account";
pub const PHARMACY_INFO_ACCOUNT_SEED: &str = "pharmacy_info_account";
pub const USER_ATA_SEED: &str = "user_ATA";
pub const PHARMACY_ATA_SEED: &str = "pharmacy_ATA";



#[account]
pub struct PharmacyInfoAccount {
    pub bump: u8,
    pub sol_ata_bump: u8,
    pub authority: Pubkey,
    license_number: u32,
    phone_number: u32,
}

impl PharmacyInfoAccount {
    pub fn new(
        &mut self,
        bump: u8,
        sol_ata_bump: u8,
        authority: Pubkey,
        license_number: u32,
        phone_number: u32
    ) -> Result<()> {
        self.bump = bump;
        self.sol_ata_bump = sol_ata_bump;
        self.authority = authority;
        self.license_number = license_number;
        self.phone_number = phone_number;
        Ok(())
    }
}

#[account]
pub struct Drug {
    pub price: u64,
    pub temperature: i8,
    pub production_date: i64,
    pub batch_number: u32,
}

impl Drug {
    pub fn new(&mut self, price: u64, temperature: i8, batch_number: u32) -> Result<()> {
        self.price = price;
        self.temperature = temperature;
        self.production_date = Clock::get()?.unix_timestamp;
        self.batch_number = batch_number;
        Ok(())
    }
}

#[account]
pub struct UserPharmacyAssociatedAccount {
    pub bump: u8,
    pub sol_ata_bump: u8,
    pub authority: Pubkey,
    pub pharmacy_infos: [PharmacyInfo; MAX_PHARMACIES],
}

impl UserPharmacyAssociatedAccount {
    pub fn new(&mut self, authority: Pubkey, bump: u8, sol_ata_bump: u8) -> Result<()> {
        self.bump = bump;
        self.sol_ata_bump = sol_ata_bump;
        self.authority = authority;
        Ok(())
    }

    pub fn add_pharmacy_info(&mut self, pharmacy_pk: &Pubkey) -> Result<()> {
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_infos[i].pharmacy_pk == Pubkey::default() {
                self.pharmacy_infos[i].pharmacy_pk = *pharmacy_pk;
                break;
            }
        }
        Ok(())
    }

    pub fn add_order(&mut self, pharmacy_pk: &Pubkey, order: Order) -> Result<()> {
        let pharmacy = self.get_pharmacy_account(pharmacy_pk)?;
        pharmacy.add_order(order)?;
        Ok(())
    }

    pub fn check_pharmacy_info_exists(&self, pharmacy_pk: &Pubkey) -> bool {
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_infos[i].pharmacy_pk == *pharmacy_pk {
                return true;
            }
        }
        false
    }

    pub fn get_pharmacy_account(&mut self, pharmacy_pk: &Pubkey) -> Result<&mut PharmacyInfo> {
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_infos[i].pharmacy_pk == *pharmacy_pk {
                return Ok(&mut self.pharmacy_infos[i]);
            }
        }
        Err(ErrorCode::PharmacyDoesNotBound.into())
    }
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct PharmacyInfo {
    pub pharmacy_pk: Pubkey,
    pub order_history: [Order; MAX_ORDERS],
}

impl PharmacyInfo {
    pub fn loss_drug(&mut self, drug_pk: &Pubkey) -> Result<()> {
        for i in 0..MAX_ORDERS {
            let order = &mut self.order_history[i];
            if order.drug_pk == *drug_pk {
                order.set_loss();
                return Ok(());
            }
        }
        Err(ErrorCode::DrugDoesNotExist.into())
    }

    pub fn add_order(&mut self, order: Order) -> Result<()> {
        for i in 0..MAX_ORDERS {
            if self.order_history[i].drug_pk == Pubkey::default() {
                self.order_history[i] = order;
                return Ok(());
            }
        }
        Err(ErrorCode::OrderLimitExceeded.into())
    }

    pub fn sign_for(&mut self, drug_pk: &Pubkey) -> Result<()> {
        for i in 0..MAX_ORDERS {
            let order = &mut self.order_history[i];
            if order.drug_pk == *drug_pk && order.status == IN_DELIVERY {
                order.sign_for();
                return Ok(());
            }
        }
        Err(ErrorCode::DrugDoesNotExist.into())
    }

    pub fn send_out(&mut self, drug_pk: &Pubkey) -> Result<()> {
        for i in 0..MAX_ORDERS {
            let order = &mut self.order_history[i];
            if order.drug_pk == *drug_pk {
                order.send_out();
                return Ok(());
            }
        }
        Err(ErrorCode::DrugDoesNotExist.into())
    }
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Order {
    pub drug_pk: Pubkey,
    pub status: u8,
    pub drug_status: u8,
    pub created_at: i64,
}

impl Order {
    pub fn new(drug_pk: &Pubkey) -> Self {
        Self {
            drug_pk: *drug_pk,
            status: INITIAL,
            drug_status: MEDICATION_NORMAL,
            created_at: Clock::get().unwrap().unix_timestamp,
        }
    }

    pub fn set_loss(&mut self) {
        self.status = CANCELLED;
        self.drug_status = MEDICATION_LOST;
    }

    pub fn sign_for(&mut self) {
        self.status = COMPLETED;
    }

    pub fn send_out(&mut self) {
        self.status = IN_DELIVERY;
    }
}
