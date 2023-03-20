pub mod contract;
pub mod state;
pub mod responses;
pub mod error;

use contract::{ ContractQueryMsg, ContractExecMsg };
use cosmwasm_std::{ entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Deps, Binary };
use error::ContractError;

use crate::contract::{ InstantiateMsg, AdminContract };

const CONTRACT: AdminContract = AdminContract::new();

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
  msg.dispatch(&CONTRACT, (deps, env, info))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: ContractQueryMsg) -> Result<Binary, ContractError> {
  msg.dispatch(&CONTRACT, (deps, env))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ContractExecMsg) -> Result<Response, ContractError> {
  msg.dispatch(&CONTRACT, (deps, env, info))
}
