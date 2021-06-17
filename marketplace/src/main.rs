#![no_main]
extern crate marketplace;

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) =
        contract::contract_api::storage::create_contract_package_at_hash();
    let entry_points = marketplace::get_entrypoints(None);
    marketplace::deploy(entry_points, contract_package_hash);
}
