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
    let val: String = get_key("name").unwrap();
    ret(val)
}

#[no_mangle]
pub extern "C" fn ipfs_hash() {
    let val: String = get_key("ipfs_hash").unwrap();
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: PublicKey = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account.to_account_hash())).unwrap();
    ret(val)
}

#[no_mangle]
pub extern "C" fn transfer() {
    let sender: PublicKey = runtime::get_named_arg("sender");
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let piece_number: u64 = runtime::get_named_arg("piece_number");
    if sender.to_account_hash() != runtime::get_caller() {
        runtime::revert(ApiError::PermissionDenied);
    }
    let owner = owner_of(piece_number);
    if owner.is_none() {
        runtime::revert(ApiError::User(3));
    }
    if sender != owner.unwrap() {
        runtime::revert(ApiError::PermissionDenied);
    }
    set_key(&token_key(piece_number), Some(recipient));

    let sender_key: String = get_key(&balance_key(&sender.to_account_hash())).unwrap();
    let recipient_key: String = get_key(&balance_key(&recipient.to_account_hash())).unwrap();
    let new_sender_balance: U256 = get_key::<U256>(&sender_key).unwrap() - 1;
    set_key(&sender_key, new_sender_balance);
    let new_recipient_balance: U256 = get_key::<U256>(&recipient_key).unwrap() + 1;
    set_key(&recipient_key, new_recipient_balance);
}

#[no_mangle]
pub extern "C" fn mint() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let piece_number: u64 = runtime::get_named_arg("piece_number");
    let minter: PublicKey = minter();
    if minter.to_account_hash() != runtime::get_caller() {
        runtime::revert(ApiError::PermissionDenied);
    }
    let owner = owner_of(piece_number);
    if owner.is_some() {
        runtime::revert(ApiError::User(2));
    }

    set_key(&token_key(piece_number), Some(recipient));

    let recipient_key = balance_key(&recipient.to_account_hash());
    let recipient_balance = get_key::<U256>(&recipient_key).unwrap();
    set_key(&recipient_key, recipient_balance + 1);
}

#[no_mangle]
pub extern "C" fn call() {}

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

fn seed() -> URef {
    get_key::<URef>("seed").unwrap()
}

fn minter() -> PublicKey {
    get_key::<PublicKey>("minter").unwrap()
}

fn number_of_pieces() -> u64 {
    get_key::<u64>("number_of_pieces").unwrap()
}

fn owner_of(piece_number: u64) -> Option<PublicKey> {
    let token_id = to_token_id(piece_number);
    get_key::<PublicKey>(&token_key(token_id))
}

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

fn token_key(token_id: u64) -> String {
    format!("tokens_{}", to_token_id(token_id))
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
