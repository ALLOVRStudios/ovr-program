use anchor_lang::prelude::*;

#[error_code]
pub enum AllovrError {
    #[msg("Pool already exists")]
    PoolAlreadyExists,
    #[msg("Pool index does not match head")]
    PoolIndexDoesNotMatchHead,
    #[msg("Pool index is invalid (allowed: 0 - 99)")]
    InvalidPoolIndex,
}
