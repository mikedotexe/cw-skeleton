use crate::tests::{alice, contracts, default_app};
use cw_multi_test::Executor;
use cw_skeleton_pkg::msgs::instantiate_msg::InstantiateMsg;

#[test]
fn the_usual() {
    let mut app = default_app();
    let code_id = app.store_code(contracts::me());
    assert!(
        app.instantiate_contract(
            code_id,
            alice(), // from
            &InstantiateMsg { owner: None },
            &[], // funds
            "regifted the labelmaker",
            None, // No admin, so it'll be immutable
        )
        .is_ok(),
        "Should instantiate fine"
    );

    println!("To see this, you'll have to run the command:");
    println!("cargo test -- --nocapture");
    println!("or 'cargo t' which won't show warnings");
}
