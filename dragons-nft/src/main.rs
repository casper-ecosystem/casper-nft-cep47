#![no_main]

use casper_contract::contract_api::runtime::get_named_arg;
use casper_contract::contract_api::storage::create_contract_package_at_hash;

#[no_mangle]
pub extern "C" fn owner_of() {}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = create_contract_package_at_hash();
    let entry_points = cep47::get_entrypoints(None);
    cep47::deploy(
        &get_named_arg::<String>("token_name"),
        &get_named_arg::<String>("token_symbol"),
        &get_named_arg::<String>("token_uri"),
        entry_points,
        contract_package_hash,
        false,
    );
}
