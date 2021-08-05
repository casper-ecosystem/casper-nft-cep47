#![no_main]

use casper_contract::contract_api::{
    runtime::{self, call_versioned_contract},
    storage,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, EntryPoint, EntryPointAccess, EntryPoints, Key,
    RuntimeArgs,
};

#[no_mangle]
pub extern "C" fn transfer_token() {
    call_versioned_contract(
        runtime::get_named_arg("nft"),
        None,
        "transfer_token",
        runtime_args! {
            "sender" => runtime::get_named_arg::<Key>("sender"),
            "recipient"=>runtime::get_named_arg::<Key>("recipient"),
            "token_id"=>runtime::get_named_arg::<String>("token_id")
        },
    )
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _access) = storage::create_contract_package_at_hash();
    let entry_points = {
        let mut eps = EntryPoints::new();
        eps.add_entry_point(EntryPoint::new(
            "transfer_token",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps
    };
    let named_keys = NamedKeys::new();

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    // wrap the contract hash so that it can be reached from the test environment
    runtime::put_key(
        "owning_contract_hash",
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        "owning_contract_pack",
        storage::new_uref(contract_package_hash).into(),
    );
}
