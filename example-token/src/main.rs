#![no_main]
extern crate cep47;
use cep47::{endpoint, logic::CEP47Contract, ret, CasperCEP47Contract};
use types::{CLType, EntryPoints};

#[no_mangle]
pub extern "C" fn new_name() {
    let contract = CasperCEP47Contract::new();
    ret(String::from("[CEP47] ").push_str(contract.name().as_str()))
}

#[no_mangle]
pub extern "C" fn change_name() {
    // let contract = CasperCEP47Contract::new();
}

/**
It should present how to:

- override one of the CEP47 endpoints. Override the name endpoint to always return a name in the format:
[CEP47] <name>
- add a new endpoint. Add change_name that allows to change a name.
- show how to tests only the name and and change_name. If the name is changed to example-nft then name endpoint should return [CEP47] example-nft. This test should be implemented as a WASM file.
 */

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) =
    contract::contract_api::storage::create_contract_package_at_hash();
    let entry_points = cep47::get_entrypoints(None);
    let mut new_entry_points = EntryPoints::new();
    for ep in entry_points.take_entry_points().iter() {
        if ep.name().eq("name") {
            new_entry_points.add_entry_point(endpoint("new_name", vec![], CLType::String, None));
        } else {
            new_entry_points.add_entry_point(ep.to_owned());
        }
    }
    cep47::deploy(
        &contract::contract_api::runtime::get_named_arg::<String>("token_name"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_symbol"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_uri"),
        new_entry_points,
        contract_package_hash,
    );
}
