use cosmwasm_std::{ Addr, StdError };
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
  #[error("{0}")] Std(#[from] StdError),
  #[error("{sender} is not a contract admin")] Unauthorized {
    sender: Addr,
  },
}
