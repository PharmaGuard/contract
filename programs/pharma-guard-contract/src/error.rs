use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Already bound to this pharmacy")]
    PharmacyAlreadyBound,
    #[msg("Not bound to this pharmacy")]
    PharmacyDoesNotBound,
    #[msg("The drug does not exist")]
    DrugDoesNotExist,
    #[msg("The order limit has been exceeded")]
    OrderLimitExceeded,
    #[msg("The pharmacy is not associated with the drug")]
    PharmacyNotAssociated
}
