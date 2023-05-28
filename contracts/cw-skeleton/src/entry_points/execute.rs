pub mod my_execute;

use crate::errors::ContractError;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_skeleton_pkg::msgs::execute_msg::ExecuteMsg;

/// Execute entry point.
/// You may see a list of the execute variants (methods) in [ExecuteMsg](ExecuteMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MyExecute { favorite_hero } => {
            my_execute::execute(env, deps, info, favorite_hero)
        }
    }
}
