use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_skeleton_pkg::msgs::instantiate_msg::InstantiateMsg;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Validate that they sent us good addresses
    let owner = if let Some(owner_string) = msg.owner {
        deps.api.addr_validate(&owner_string)?
    } else {
        info.sender
    };

    let config = Config { owner };
    CONFIG.save(deps.storage, &config)?;

    // This sets the version, imported from cw2, just a normal thing to do
    // Boilerplate, don't worry about it
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}
