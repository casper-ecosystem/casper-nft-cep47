#![no_main]
// extern crate cep47;
// use cep47::endpoints::aa as other_aa;

// #[no_mangle]
// pub fn aa() {

// }

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) =
        contract::contract_api::storage::create_contract_package_at_hash();
    let entry_points = cep47::get_entrypoints(None);
    cep47::deploy(
        &contract::contract_api::runtime::get_named_arg::<String>("token_name"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_symbol"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_uri"),
        entry_points,
        contract_package_hash,
    );
}
