use crate::state::{Config, CONFIG};
use cosmwasm_std::{Deps, Env, StdResult};

/// Logic for the [Config](Config) (`get_value`) method
pub fn query(deps: Deps, _env: Env) -> StdResult<Config> {
    // Grab our owner
    let config = CONFIG.load(deps.storage)?;
    // Return it
    Ok(config)
}
