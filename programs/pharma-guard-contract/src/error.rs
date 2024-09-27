use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("NFT already staked")]
    AlreadyStaked,

}