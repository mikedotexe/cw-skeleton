use cosmwasm_std::{coins, Addr};
use cw_multi_test::{App, AppBuilder};

mod contracts;
mod core_tests;

/// We set this to "TOKEN" to match the denom here:
/// https://github.com/CosmWasm/cosmwasm/blob/32f308a1a56ae5b8278947891306f7a374c3df94/packages/vm/src/environment.rs#L383
pub const DENOM: &str = "TOKEN";

pub fn alice() -> Addr {
    Addr::unchecked("alice")
}
pub fn bob() -> Addr {
    Addr::unchecked("bob")
}
pub fn charlie() -> Addr {
    Addr::unchecked("charlie")
}

pub fn default_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        let accounts: Vec<(u128, Addr)> =
            vec![(6_000_000, alice()), (660_000, bob()), (66_600, charlie())];
        for (amt, address) in accounts {
            router
                .bank
                .init_balance(storage, &address, coins(amt, DENOM))
                .unwrap();
        }
    })
}
