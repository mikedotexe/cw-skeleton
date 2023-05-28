use cosmwasm_schema::cw_serde;

/// The instantiate message has an optional owner
#[cw_serde]
pub struct InstantiateMsg {
    /// By default will assign the sender as the owner
    /// You may also make another address owner with this.
    pub owner: Option<String>,
}
