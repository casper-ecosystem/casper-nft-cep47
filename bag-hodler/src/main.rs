#![no_std]
#![no_main]

extern crate alloc;
use casper_types::{EntryPoints, ContractHash, Key, EntryPoint, Parameter, CLType, EntryPointAccess, EntryPointType, runtime_args, RuntimeArgs};
use casper_contract::{unwrap_or_revert::UnwrapOrRevert, contract_api::{runtime, storage}};
use alloc::{vec, string::String};

#[no_mangle]
pub extern "C" fn transfer_back() {
    let token_contract_hash = ContractHash::new(runtime::get_named_arg::<Key>("token_contract_hash").into_hash().unwrap_or_revert());
    let sender_key = runtime::get_named_arg::<Key>("sender");
    let recipient_key = runtime::get_named_arg::<Key>("recipient");

    runtime::call_contract(
        token_contract_hash,
        "transfer_token",
        runtime_args! {
            "sender" => sender_key,
            "recipient" => recipient_key,
            "token_id" => runtime::get_named_arg::<String>("token_id"),
        },
    )
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        String::from("transfer_back"),
        vec![
            Parameter::new("token_contract_hash", CLType::Key),
            Parameter::new("sender", CLType::Key),
            Parameter::new("recipient", CLType::Key),
            Parameter::new("token_id", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let _ = storage::new_contract(
        entry_points,
        None,
        Some(String::from("bag_hodler_package_hash")),
        Some(String::from("bag_hodler_access_token")),
    );
}