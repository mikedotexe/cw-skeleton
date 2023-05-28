use cosmwasm_schema::cw_serde;

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    MyExecute { favorite_hero: String },
}
