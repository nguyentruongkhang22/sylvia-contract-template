use anyhow::{ bail, Result as AnyResult };
use cosmwasm_std::{ Empty, from_slice };
use cw_multi_test::Contract;

use crate::contract::{ AdminContract, ContractExecMsg, InstantiateMsg, ContractQueryMsg };

impl Contract<Empty> for AdminContract<'_> {
  fn execute(
    &self,
    deps: cosmwasm_std::DepsMut<Empty>,
    env: cosmwasm_std::Env,
    info: cosmwasm_std::MessageInfo,
    msg: Vec<u8>
  ) -> AnyResult<cosmwasm_std::Response<Empty>> {
    from_slice::<ContractExecMsg>(&msg)?.dispatch(self, (deps, env, info)).map_err(Into::into)
  }

  fn instantiate(
    &self,
    deps: cosmwasm_std::DepsMut<Empty>,
    env: cosmwasm_std::Env,
    info: cosmwasm_std::MessageInfo,
    msg: Vec<u8>
  ) -> AnyResult<cosmwasm_std::Response<Empty>> {
    from_slice::<InstantiateMsg>(&msg)?.dispatch(self, (deps, env, info)).map_err(Into::into)
  }

  fn query(
    &self,
    deps: cosmwasm_std::Deps<Empty>,
    env: cosmwasm_std::Env,
    msg: Vec<u8>
  ) -> AnyResult<cosmwasm_std::Binary> {
    from_slice::<ContractQueryMsg>(&msg)?.dispatch(self, (deps, env)).map_err(Into::into)
  }

  fn sudo(
    &self,
    _deps: cosmwasm_std::DepsMut<Empty>,
    _env: cosmwasm_std::Env,
    _msg: Vec<u8>
  ) -> AnyResult<cosmwasm_std::Response<Empty>> {
    bail!("sudo not implemented for contract")
  }

  fn reply(
    &self,
    _deps: cosmwasm_std::DepsMut<Empty>,
    _env: cosmwasm_std::Env,
    _msg: cosmwasm_std::Reply
  ) -> AnyResult<cosmwasm_std::Response<Empty>> {
    bail!("reply not implemented for contract")
  }

  fn migrate(
    &self,
    _deps: cosmwasm_std::DepsMut<Empty>,
    _env: cosmwasm_std::Env,
    _msg: Vec<u8>
  ) -> AnyResult<cosmwasm_std::Response<Empty>> {
    bail!("reply not implemented for contract")
  }
}

#[cfg(test)]
mod tests {
  use cosmwasm_std::from_binary;
  use cosmwasm_std::testing::{ mock_dependencies, mock_env, mock_info };

  use crate::contract::{ InstantiateMsg, QueryMsg, ContractQueryMsg };
  use crate::responses::AdminListResp;
  use crate::{ instantiate, query };

  #[test]
  fn admin_list_query() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(deps.as_mut(), env.clone(), mock_info("sender", &[]), InstantiateMsg {
      admins: vec!["admin1".to_owned(), "admin2".to_owned()],
    }).unwrap();

    let msg = QueryMsg::AdminList {};
    let resp = query(deps.as_ref(), env, ContractQueryMsg::AdminContract(msg)).unwrap();
    let resp: AdminListResp = from_binary(&resp).unwrap();
    println!(" -- resp: {:?}", resp);
    assert_eq!(resp, AdminListResp {
      admins: vec!["admin1".to_owned(), "admin2".to_owned()],
    });
  }
}