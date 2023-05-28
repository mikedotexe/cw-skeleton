//! The errors that can be thrown for this boolean contract, including demonstration ones.

use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    /// Example error with placeholder
    #[error("Unauthorized, method can only be called by {:?}", owner)]
    Unauthorized { owner: Addr },
}
