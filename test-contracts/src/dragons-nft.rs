#![no_main]

use casper_contract::contract_api::runtime::{self, get_named_arg};
use casper_contract::contract_api::storage::create_contract_package_at_hash;

#[no_mangle]
pub extern "C" fn call() {
    let token_name: String = get_named_arg("token_name");
    let token_symbol: String = get_named_arg("token_symbol");
    let token_meta: cep47::Meta = get_named_arg("token_meta");

    let (contract_package_hash, access_token) = create_contract_package_at_hash();
    let entry_points = cep47::get_entrypoints(Some(contract_package_hash));

    cep47::deploy(
        token_name.clone(),
        token_symbol,
        token_meta,
        entry_points,
        contract_package_hash,
        false,
    );

    runtime::put_key(
        &format!("{}_package_hash", token_name),
        contract_package_hash.into(),
    );
    runtime::put_key(&format!("{}_access_token", token_name), access_token.into());
}
