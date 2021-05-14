#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    ApiError, AsymmetricType, CLTyped, CLValue, PublicKey, URef, U256,
};

/**
 * ApiError::User(1) - The number of piece is out or range.
 * ApiError::User(2) - The piece of NFT is already minted and owned by someone.
 * ApiError::User(3) - The piece of NFT is not minted yet.
 */

#[derive(Hash)]
pub struct TokenId {
    seed: URef,
    piece_number: u64,
}

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn ipfs_hash() {
    let val: String = get_key("ipfs_hash");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn transfer() {
    let sender: AccountHash = runtime::get_named_arg("sender");
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let piece_number: u64 = runtime::get_named_arg("piece_number");
    if sender != runtime::get_caller() {
        runtime::revert(ApiError::PermissionDenied);
    }

    let total_number = number_of_pieces();
    if piece_number > total_number || piece_number == 0 {
        runtime::revert(ApiError::User(1));
    }

    let owner = owner_of(piece_number);
    if owner.is_none() {
        runtime::revert(ApiError::User(3));
    }
    if sender != owner.unwrap() {
        runtime::revert(ApiError::PermissionDenied);
    }

    let mut owners = owners();
    let token_id = to_token_id(piece_number);
    owners.insert(token_id, Some(recipient));
    set_key("owners", owners);

    let sender_key: String = get_key(&balance_key(&sender));
    let recipient_key: String = get_key(&balance_key(&recipient));
    let new_sender_balance: U256 = get_key::<U256>(&sender_key) - 1;
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = get_key::<U256>(&recipient_key) + 1;
    set_key(&recipient_key, new_recipient_balance);
}

#[no_mangle]
pub extern "C" fn mint() {
    let recipient: AccountHash = runtime::get_named_arg("recipient");
    let piece_number: u64 = runtime::get_named_arg("piece_number");
    let minter: AccountHash = minter();
    if minter != runtime::get_caller() {
        runtime::revert(ApiError::PermissionDenied);
    }
    let token_id = to_token_id(piece_number);
    let owner = owner_of(token_id);
    if owner.is_some() {
        runtime::revert(ApiError::User(2));
    }
    let mut owners = owners();
    owners.insert(token_id, Some(recipient));
    set_key("owners", owners);

    let recipient_key = balance_key(&recipient);
    let recipient_balance = get_key::<U256>(&recipient_key);
    set_key(&recipient_key, recipient_balance + 1);
}

#[no_mangle]
pub extern "C" fn call() {}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
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

fn seed() -> URef {
    let val: URef = get_key("seed");
    return val;
}

fn minter() -> AccountHash {
    let val: AccountHash = get_key("minter");
    return val;
}

fn number_of_pieces() -> u64 {
    let val: u64 = get_key("number_of_pieces");
    return val;
}

fn owners() -> BTreeMap<u64, Option<AccountHash>> {
    let val: BTreeMap<u64, Option<AccountHash>> = get_key("owners");
    return val;
}

fn owner_of(token_id: u64) -> Option<AccountHash> {
    let owners = owners();
    let owner = owners.get(&token_id).unwrap();
    owner.clone()
}

fn to_token_id(piece_number: u64) -> u64 {
    let total_pieces: u64 = number_of_pieces();
    if piece_number > total_pieces || piece_number == 0 {
        runtime::revert(ApiError::User(1));
    }

    let mut hasher = DefaultHasher::new();
    let token_id = TokenId {
        seed: seed(),
        piece_number,
    };
    Hash::hash(&token_id, &mut hasher);
    hasher.finish()
}

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}
