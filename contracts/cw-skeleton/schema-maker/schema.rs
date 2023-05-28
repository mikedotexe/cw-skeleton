use cosmwasm_schema::write_api;

use cw_skeleton::msgs::ExecuteMsg;
use cw_skeleton::msgs::InstantiateMsg;
use cw_skeleton::msgs::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
