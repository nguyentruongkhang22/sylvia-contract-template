use cosmwasm_schema::write_api;
use cw_template::contract::{ ExecMsg, QueryMsg, InstantiateMsg };

fn main() {
  write_api! {
    instantiate: InstantiateMsg,
    execute: ExecMsg,
    query: QueryMsg,
  }
}