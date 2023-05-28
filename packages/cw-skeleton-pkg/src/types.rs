use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Config {
    /// Some, most, or all methods check this address for privileged access
    pub owner: Addr,
}
