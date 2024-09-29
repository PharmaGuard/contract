use crate::error::ErrorCode;
use anchor_lang::prelude::*;

pub const INITIAL: u8 = 1 << 0;
pub const IN_DELIVERY: u8 = 1 << 1;
pub const CANCELLED: u8 = 1 << 2;
pub const COMPLETED: u8 = 1 << 3;

pub const MEDICATION_NORMAL: u8 = 1 << 4;
pub const MEDICATION_LOST: u8 = 1 << 5;

const MAX_ORDERS: usize = 3;
const MAX_PHARMACIES: usize = 3;

#[account]
pub struct Pharmacy {
    license_number: u32,
    phone_number: u32,
}

impl Pharmacy {
    pub fn new(&mut self, license_number: u32, phone_number: u32) -> Result<()> {
        self.license_number = license_number;
        self.phone_number = phone_number;
        Ok(())
    }
}

#[account]
pub struct Drug {
    pub temperature: i8,
    pub production_date: i64,
    pub batch_number: u32,
}

impl Drug {
    pub fn new(&mut self, temperature: i8, batch_number: u32) -> Result<()> {
        self.temperature = temperature;
        self.production_date = Clock::get()?.unix_timestamp;
        self.batch_number = batch_number;
        Ok(())
    }
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub pharmacy_account_list: [PharmacyAccount; MAX_PHARMACIES],
}

impl User {
    pub fn new(&mut self, authority: Pubkey) -> Result<()> {
        self.authority = authority;
        Ok(())
    }

    pub fn add_pharmacy_account(&mut self, pharmacy_pk: &Pubkey) -> Result<()> {
        // check whether the pharmacy account already exists
        // if !self.check_pharmacy_account_exists(pharmacy_pk) {
        //     return Err(ErrorCode::PharmacyAccountAlreadyExists.into());
        // }
        // add the pharmacy account to the list
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_account_list[i].pharmacy_pk == Pubkey::default() {
                self.pharmacy_account_list[i].pharmacy_pk = *pharmacy_pk;
                break;
            }
        }

        Ok(())
    }

    pub fn add_order(&mut self, pharmacy_pk: &Pubkey, order: Order) -> Result<()> {
        let pharmacy = self.get_pharmacy_account(pharmacy_pk);
        if let Some(pharmacy) = pharmacy {
            pharmacy.add_order(order)?;
        } else {
            return Err(ErrorCode::PharmacyAccountDoesNotExist.into());
        }
        Ok(())
    }

    pub fn check_pharmacy_account_exists(&self, pharmacy_pk: &Pubkey) -> bool {
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_account_list[i].pharmacy_pk == *pharmacy_pk {
                return true;
            }
        }
        false
    }

    pub fn get_pharmacy_account(&mut self, pharmacy_pk: &Pubkey) -> Option<&mut PharmacyAccount> {
        for i in 0..MAX_PHARMACIES {
            if self.pharmacy_account_list[i].pharmacy_pk == *pharmacy_pk {
                return Some(&mut self.pharmacy_account_list[i]);
            }
        }
        None
    }

}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct PharmacyAccount {
    pub pharmacy_pk: Pubkey,
    pub order_history: [Order; MAX_ORDERS],
}

impl PharmacyAccount {
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
        self.drug_status = MEDICATION_LOST;
    }

    pub fn sign_for(&mut self) {
        self.status = COMPLETED;
    }

    pub fn send_out(&mut self) {
        self.status = IN_DELIVERY;
    }
}
