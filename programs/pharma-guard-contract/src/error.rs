use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The expiration time should be greater than the current time")]
    IllegalExpirationTime,
    #[msg("The medication is not expired and cannot be deleted")]
    NotExpired,
    #[msg("The maximum number of medications has been reached.")]
    MaxMedicationsReached,
    #[msg("The index is out of bounds")]
    InvalidIndex,
    #[msg("The owner is not the same as the manufacturer owner")]
    NotOwner,
    #[msg("The medication is not initialized")]
    MedicationNotInitialized,
    #[msg("No empty medication available")]
    NoEmptyMedicationAvailable,
    #[msg("The pharmacy account already exists")]
    PharmacyAccountAlreadyExists,
    #[msg("The pharmacy account does not exist")]
    PharmacyAccountDoesNotExist,
    #[msg("The drug does not exist")]
    DrugDoesNotExist,
    #[msg("The order limit has been exceeded")]
    OrderLimitExceeded
}
