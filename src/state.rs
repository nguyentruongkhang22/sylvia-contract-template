use cosmwasm_std::{ StdResult, Response };

use crate::contract::AdminContract;
use crate::error::ContractError;
use crate::responses::AdminListResp;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(sylvia::serde::Serialize, sylvia::serde::Deserialize, Clone, Debug, PartialEq, sylvia::schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
  pub admins: Vec<String>,
}
impl InstantiateMsg {
  pub fn dispatch(
    self,
    contract: &AdminContract<'_>,
    ctx: (cosmwasm_std::DepsMut, cosmwasm_std::Env, cosmwasm_std::MessageInfo)
  ) -> StdResult<Response> {
    let Self { admins } = self;
    contract.instantiate(ctx.into(), admins).map_err(Into::into)
  }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
  sylvia::serde::Serialize,
  sylvia::serde::Deserialize,
  Clone,
  Debug,
  PartialEq,
  sylvia::schemars::JsonSchema,
  cosmwasm_schema::QueryResponses
)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  #[returns(AdminListResp)] AdminList {},
}

impl QueryMsg {
  pub fn dispatch(
    self,
    contract: &AdminContract,
    ctx: (cosmwasm_std::Deps, cosmwasm_std::Env)
  ) -> std::result::Result<sylvia::cw_std::Binary, ContractError> {
    use QueryMsg::*;
    match self {
      AdminList {} => { cosmwasm_std::to_binary(&contract.admin_list(ctx.into())?).map_err(Into::into) }
    }
  }
}
