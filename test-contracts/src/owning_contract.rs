#![no_main]

use casper_contract::{
    contract_api::{
        runtime::{self},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, CLType, EntryPoint, EntryPointAccess, EntryPoints, Key, Parameter,
};

#[no_mangle]
pub extern "C" fn call_back() {
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _access) = storage::create_contract_package_at_hash();
    let entry_points = {
        let mut eps = EntryPoints::new();
        eps.add_entry_point(EntryPoint::new(
            "call_back",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps
    };
    let named_keys = {
        let mut nk = NamedKeys::new();
        nk
    };

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    // wrap the contract hash so that it can be reached from the test environment
    runtime::put_key(
        "owning_contract_hash",
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        "owning_contract_package",
        storage::new_uref(contract_package_hash).into(),
    );
}
