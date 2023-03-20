use cosmwasm_std::{ Addr, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Deps, Order };
use cw_storage_plus::Map;
use schemars;
use sylvia::contract;
use crate::{ error::ContractError, responses::AdminListResp };

pub struct AdminContract<'a> {
  pub(crate) admins: Map<'a, &'a Addr, Empty>,
}

#[contract]
impl AdminContract<'_> {
  pub const fn new() -> Self {
    Self {
      admins: Map::new("admins"),
    }
  }

  #[msg(instantiate)]
  pub fn instantiate(&self, ctx: (DepsMut, Env, MessageInfo), admins: Vec<String>) -> StdResult<Response> {
    let (deps, _, _) = ctx;

    for admin in admins {
      let admin = deps.api.addr_validate(&admin)?;
      self.admins.save(deps.storage, &admin, &(Empty {}))?;
    }

    Ok(Response::new())
  }

  #[msg(query)]
  pub fn admin_list(&self, ctx: (Deps, Env)) -> StdResult<AdminListResp> {
    let (deps, _) = ctx;

    let admins: Result<_, _> = self.admins
      .keys(deps.storage, None, None, Order::Ascending)
      .map(|addr| addr.map(String::from))
      .collect();

    Ok(AdminListResp { admins: admins? })
  }

  #[msg(exec)]
  pub fn add_member(&self, ctx: (DepsMut, Env, MessageInfo), admin: String) -> Result<Response, ContractError> {
    let (deps, _, info) = ctx;

    if !self.admins.has(deps.storage, &info.sender) {
      return Err(ContractError::Unauthorized {
        sender: info.sender,
      });
    }
    let admin = deps.api.addr_validate(&admin)?;
    self.admins.save(deps.storage, &admin, &(Empty {}))?;

    Ok(Response::new())
  }
}
