//! Execute my logic

use crate::errors::ContractError;
use crate::state::CONFIG;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Logic for the [MyExecute](cw_skeleton_pkg::msgs::execute_msg::ExecuteMsg::MyExecute) (`my_execute`) method
pub fn execute(
    _env: Env,
    deps: DepsMut,
    info: MessageInfo,
    favorite_hero: String,
) -> Result<Response, ContractError> {
    // Grab our owner
    let config = CONFIG.load(deps.storage)?;
    let owner = config.owner;

    // Access control, so only owner can proceed
    if info.sender != owner {
        return Err(ContractError::Unauthorized { owner });
    }

    Ok(Response::new()
        .add_attribute("action", "my_execute")
        .add_attribute("favorite_hero", favorite_hero))
}
