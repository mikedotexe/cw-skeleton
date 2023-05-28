use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

// Useful in cw-multi-test with other contracts.
pub(crate) fn me() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry_points::execute::execute,
        crate::entry_points::instantiate::instantiate,
        crate::entry_points::query::query,
    );
    Box::new(contract)
}
