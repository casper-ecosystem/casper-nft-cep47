#![no_main]
extern crate cep47;

#[no_mangle]
pub extern "C" fn call() {
    cep47::deploy(
        &contract::contract_api::runtime::get_named_arg::<String>("token_name"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_symbol"),
        &contract::contract_api::runtime::get_named_arg::<String>("token_uri")
    );
}