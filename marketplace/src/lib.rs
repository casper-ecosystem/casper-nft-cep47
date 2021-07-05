#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod offer;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use contract::contract_api::runtime::revert;
use contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;
use offer::Offer;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use types::bytesrepr::Error;
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    AccessRights, ApiError, AsymmetricType, CLType, CLTyped, CLValue, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, PublicKey, URef,
    U256, U512,
};

#[no_mangle]
pub extern "C" fn put_on_sale_test() {
    let sel = runtime::get_named_arg("seller");
    let offer = Offer::test_struct(sel);
    let rets = offer.store();
    ret(rets)
}

#[no_mangle]
pub extern "C" fn test_buy() {
    let offer_key: String =
        "018a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c_test_order".to_string();
    let purse: URef = runtime::get_named_arg("purse");
    let offer: Offer = Offer::load(&offer_key);
    if system::get_purse_balance(purse).unwrap_or_revert() == offer.price
        && runtime::get_caller() != offer.seller.to_account_hash()
    {
        system::transfer_from_purse_to_account(
            purse,
            offer.seller.to_account_hash(),
            system::get_purse_balance(purse).unwrap_or_revert(),
            None,
        )
        .unwrap_or_revert();
        remove_key(&offer_key);
    }
}

#[no_mangle]
pub extern "C" fn put_on_sale() {
    let offer = Offer::new(
        runtime::get_named_arg("seller"),
        runtime::get_named_arg("price"),
        runtime::get_named_arg("item"),
        runtime::get_named_arg("designation"),
    );
    ret(offer.store())
}

#[no_mangle]
pub extern "C" fn buy() {
    let offer_key: String = runtime::get_named_arg("offer_key");
    let payment: U512 = runtime::get_named_arg("payment");
    let offer: Offer = Offer::load(&offer_key);
    if payment == offer.price && runtime::get_caller() != offer.seller.to_account_hash() {
        system::transfer_to_account(offer.seller.to_account_hash(), payment, None)
            .unwrap_or_revert();
        remove_key(&offer_key);
        ret(offer.item)
    }
}

#[no_mangle]
pub extern "C" fn get_price() {
    let offer_key: String = runtime::get_named_arg("offer_key");
    let offer: Offer = Offer::load(&offer_key);
    ret(offer.price)
}

#[no_mangle]
pub extern "C" fn cancel() {
    let offer_key: String = runtime::get_named_arg("offer_key");
    let offer: Offer = Offer::load(&offer_key);
    if runtime::get_caller() == offer.seller.to_account_hash() {
        remove_key(&offer_key);
    }
}

pub fn get_entrypoints(package_hash: Option<ContractPackageHash>) -> EntryPoints {
    let _secure = if let Some(contract_package_hash) = package_hash {
        let deployer_group = storage::create_contract_user_group(
            contract_package_hash,
            "deployer",
            1,
            BTreeSet::default(),
        )
        .unwrap_or_revert();
        runtime::put_key("deployer_access", types::Key::URef(deployer_group[0]));
        true
    } else {
        false
    };

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint(
        "put_on_sale",
        vec![
            Parameter::new("seller", CLType::PublicKey),
            Parameter::new("designation", CLType::String),
            Parameter::new("item", CLType::URef),
            Parameter::new("price", CLType::U512),
        ],
        CLType::String,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "put_on_sale_test",
        vec![Parameter::new("seller", CLType::PublicKey)],
        CLType::String,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "buy",
        vec![
            Parameter::new("offer_key", CLType::String),
            Parameter::new("payment", CLType::U512),
        ],
        CLType::URef,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "test_buy",
        vec![Parameter::new("payment", CLType::U512)],
        CLType::URef,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "cancel",
        vec![Parameter::new("offer_key", CLType::String)],
        CLType::Unit,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "get_price",
        vec![Parameter::new("offer_key", CLType::String)],
        CLType::U512,
        None,
    ));

    entry_points
}

pub fn deploy(entry_points: EntryPoints, contract_package_hash: ContractPackageHash) {
    let named_keys = NamedKeys::new();
    // named_keys.insert("name".to_string(), storage::new_uref(token_name).into());

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key("marketplace_contract_hash", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key(
        "marketplace_contract_package_hash",
        contract_hash_pack.into(),
    );
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn remove_key(name: &str) {
    match runtime::get_key(name) {
        Some(_) => {
            runtime::remove_key(name);
        }
        None => {}
    }
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType, access: Option<&str>) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        match access {
            None => EntryPointAccess::Public,
            Some(access_key) => EntryPointAccess::groups(&[access_key]),
        },
        EntryPointType::Contract,
    )
}
