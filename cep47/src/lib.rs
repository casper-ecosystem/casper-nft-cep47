#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    AccessRights, ApiError, AsymmetricType, CLType, CLTyped, CLValue, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, HashAddr, Key, Parameter, URef,
    U256,
};
pub use cep47_logic::Meta;
use cep47_logic::{CEP47Contract, CEP47Storage, TokenId, WithStorage};

use core::convert::TryInto;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::AddAssign,
};

mod cep47_storage;
mod data;
mod entrypoints;

pub use cep47_storage::CasperCEP47Storage;
pub use entrypoints::get_entrypoints;

use data::{Balances, Metadata, Owners};

#[derive(Default)]
pub struct CasperCEP47Contract {
    storage: CasperCEP47Storage,
}

impl CasperCEP47Contract {
    pub fn new() -> CasperCEP47Contract {
        CasperCEP47Contract {
            storage: CasperCEP47Storage::new(),
        }
    }
}

impl WithStorage<CasperCEP47Storage> for CasperCEP47Contract {
    fn storage(&self) -> &CasperCEP47Storage {
        &self.storage
    }
    fn storage_mut(&mut self) -> &mut CasperCEP47Storage {
        &mut self.storage
    }
}

impl CEP47Contract<CasperCEP47Storage> for CasperCEP47Contract {}

#[cfg(not(feature = "no_name"))]
#[no_mangle]
pub extern "C" fn name() {
    let contract = CasperCEP47Contract::new();
    ret(contract.name())
}

#[cfg(not(feature = "no_symbol"))]
#[no_mangle]
pub extern "C" fn symbol() {
    let contract = CasperCEP47Contract::new();
    ret((contract.symbol(), 42u8))
}

#[cfg(not(feature = "no_meta"))]
#[no_mangle]
pub extern "C" fn meta() {
    let contract = CasperCEP47Contract::new();
    ret(contract.meta())
}

#[cfg(not(feature = "no_balance_of"))]
#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let contract = CasperCEP47Contract::new();
    ret(contract.balance_of(&account))
}

#[cfg(not(feature = "no_owner_of"))]
#[no_mangle]
pub extern "C" fn owner_of() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let contract = CasperCEP47Contract::new();
    ret(contract.owner_of(&token_id))
}

#[cfg(not(feature = "no_total_supply"))]
#[no_mangle]
pub extern "C" fn total_supply() {
    let contract = CasperCEP47Contract::new();
    ret(contract.total_supply())
}

#[cfg(not(feature = "no_token_meta"))]
#[no_mangle]
pub extern "C" fn token_meta() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let contract = CasperCEP47Contract::new();
    ret(contract.token_meta(&token_id))
}

#[cfg(not(feature = "no_is_paused"))]
#[no_mangle]
pub extern "C" fn is_paused() {
    let contract = CasperCEP47Contract::new();
    ret(contract.is_paused())
}

#[cfg(not(feature = "no_pause"))]
#[no_mangle]
pub extern "C" fn pause() {
    let mut contract = CasperCEP47Contract::new();
    contract.pause();
}

#[cfg(not(feature = "no_unpause"))]
#[no_mangle]
pub extern "C" fn unpause() {
    let mut contract = CasperCEP47Contract::new();
    contract.unpause();
}

#[cfg(not(feature = "no_mint_one"))]
#[no_mangle]
pub extern "C" fn mint_one() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let token_id: Option<TokenId> = runtime::get_named_arg("token_id");
    let token_meta: Meta = runtime::get_named_arg("token_meta");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_one(&recipient, token_id, token_meta)
        .unwrap_or_revert();
}

#[cfg(not(feature = "no_mint_many"))]
#[no_mangle]
pub extern "C" fn mint_many() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let token_ids: Option<Vec<TokenId>> = runtime::get_named_arg("token_ids");
    let token_metas: Vec<Meta> = runtime::get_named_arg("token_metas");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_many(&recipient, token_ids, token_metas)
        .unwrap_or_revert();
}

#[cfg(not(feature = "no_mint_copies"))]
#[no_mangle]
pub extern "C" fn mint_copies() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let token_ids: Option<Vec<TokenId>> = runtime::get_named_arg("token_ids");
    let token_meta: Meta = runtime::get_named_arg("token_meta");
    let count: u32 = runtime::get_named_arg("count");
    let mut contract = CasperCEP47Contract::new();
    contract
        .mint_copies(&recipient, token_ids, token_meta, count)
        .unwrap_or_revert();
}

#[cfg(not(feature = "no_burn_many"))]
#[no_mangle]
pub extern "C" fn burn_many() {
    let owner: Key = runtime::get_named_arg("owner");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_many(&owner, token_ids).unwrap_or_revert();
}

#[cfg(not(feature = "no_burn_one"))]
#[no_mangle]
pub extern "C" fn burn_one() {
    let owner: Key = runtime::get_named_arg("owner");
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_one(&owner, token_id).unwrap_or_revert();
}

#[cfg(not(feature = "no_transfer_token"))]
#[no_mangle]
pub extern "C" fn transfer_token() {
    let sender: Key = get_caller();
    let recipient: Key = runtime::get_named_arg("recipient");
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let mut contract = CasperCEP47Contract::new();
    contract
        .transfer_token(&sender, &recipient, &token_id)
        .unwrap_or_revert();
}

#[cfg(not(feature = "no_transfer_many_tokens"))]
#[no_mangle]
pub extern "C" fn transfer_many_tokens() {
    let sender: Key = get_caller();
    let recipient: Key = runtime::get_named_arg("recipient");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let mut contract = CasperCEP47Contract::new();
    contract
        .transfer_many_tokens(&sender, &recipient, &token_ids)
        .unwrap_or_revert();
}

#[cfg(not(feature = "no_update_token_metadata"))]
#[no_mangle]
pub extern "C" fn update_token_metadata() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let meta: Meta = runtime::get_named_arg("token_meta");
    let mut contract = CasperCEP47Contract::new();
    contract
        .update_token_metadata(token_id, meta)
        .unwrap_or_revert();
}

pub fn deploy(
    token_name: String,
    token_symbol: String,
    token_meta: Meta,
    entry_points: EntryPoints,
    contract_package_hash: ContractPackageHash,
    paused: bool,
) {
    // Get named keys for the contract.
    let named_keys = data::initial_named_keys(
        contract_package_hash,
        &token_name,
        &token_symbol,
        token_meta,
        paused,
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    // Save contracts.
    runtime::put_key(
        format!("{}_contract", &token_name).as_str(),
        contract_hash.into(),
    );

    // Save contract hashs.
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key(
        format!("{}_contract_hash", token_name).as_str(),
        contract_hash_pack.into(),
    );
}

pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_caller() -> Key {
    let mut callstack = runtime::get_call_stack();
    callstack.pop();
    match callstack.last().unwrap_or_revert() {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => (*contract_package_hash).into(),
    }
}
