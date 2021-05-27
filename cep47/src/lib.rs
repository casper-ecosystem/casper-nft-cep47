#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod logic;
pub mod tests;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;
use logic::{CEP47Contract, CEP47Storage, TokenId, WithStorage, URI};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    AccessRights, ApiError, AsymmetricType, CLType, CLTyped, CLValue, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, PublicKey, URef,
    U256,
};

struct CasperCEP47Storage {}
impl CasperCEP47Storage {
    pub fn new() -> CasperCEP47Storage {
        CasperCEP47Storage {}
    }
}
impl CEP47Storage for CasperCEP47Storage {
    // Metadata.
    fn name(&self) -> String {
        get_key::<String>("name").unwrap()
    }
    fn symbol(&self) -> String {
        get_key::<String>("symbol").unwrap()
    }
    fn uri(&self) -> URI {
        get_key::<URI>("uri").unwrap()
    }

    // Getters
    fn balance_of(&self, owner: PublicKey) -> U256 {
        let owner_balance = get_key::<U256>(&balance_key(&owner.to_account_hash()));
        if owner_balance.is_none() {
            U256::from(0)
        } else {
            owner_balance.unwrap()
        }
    }
    fn onwer_of(&self, token_id: TokenId) -> Option<PublicKey> {
        get_key::<PublicKey>(&owner_key(&token_id))
    }
    fn total_supply(&self) -> U256 {
        get_key::<U256>("total_supply").unwrap()
    }
    fn token_uri(&self, token_id: TokenId) -> Option<URI> {
        get_key::<URI>(&uri_key(&token_id))
    }

    // Setters
    fn get_tokens(&self, owner: PublicKey) -> Vec<TokenId> {
        let owner_tokens = get_key::<Vec<TokenId>>(&token_key(&owner.to_account_hash()));
        if owner_tokens.is_none() {
            Vec::<TokenId>::new()
        } else {
            owner_tokens.unwrap()
        }
    }
    fn set_tokens(&mut self, owner: PublicKey, token_ids: Vec<TokenId>) {
        let owner_prev_balance = self.balance_of(owner);
        let owner_new_balance = U256::from(token_ids.len() as u64);
        let prev_total_supply = self.total_supply();

        let owner_tokens = self.get_tokens(owner);
        for token_id in owner_tokens.clone() {
            remove_key(&owner_key(&token_id));
        }
        for token_id in token_ids.clone() {
            set_key(&owner_key(&token_id), owner);
        }
        set_key(&token_key(&owner.to_account_hash()), token_ids);
        set_key(&balance_key(&owner.to_account_hash()), owner_new_balance);
        set_key(
            "total_supply",
            prev_total_supply - owner_prev_balance + owner_new_balance,
        );
    }
    fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>) {
        let mut recipient_tokens = self.get_tokens(recipient);
        let mut recipient_balance = self.balance_of(recipient);
        let mut total_supply = self.total_supply();
        let uri = self.uri();

        let mut hasher = DefaultHasher::new();
        for token_uri in token_uris.clone() {
            let token_info = (total_supply, uri.clone(), token_uri.clone());
            Hash::hash(&token_info, &mut hasher);

            let token_id = TokenId::from(hasher.finish().to_string());
            recipient_tokens.push(token_id.clone());
            total_supply = total_supply + 1;
            set_key(&uri_key(&token_id), token_uri);
            set_key(&owner_key(&token_id), recipient);
        }
        recipient_balance = recipient_balance + U256::from(token_uris.len() as u64);
        set_key(
            &balance_key(&recipient.to_account_hash()),
            recipient_balance,
        );
        set_key(&token_key(&recipient.to_account_hash()), recipient_tokens);
        set_key("total_supply", total_supply);
    }
    fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256) {
        let token_uris: Vec<URI> = vec![token_uri; count.as_usize()];
        self.mint_many(recipient, token_uris);
    }
    fn new_uref(&mut self, token_id: TokenId) -> Option<URef> {
        None
    }
    fn del_uref(&mut self, token_uref: URef) -> Option<TokenId> {
        None
    }
    fn token_id(&self, token_uref: URef) -> Option<TokenId> {
        None
    }
}
struct CasperCEP47Contract {
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
/**
 * ApiError::User(1) - The number of piece is out or range.
 * ApiError::User(2) - The piece of NFT is already minted and owned by someone.
 * ApiError::User(3) - The piece of NFT is not minted yet.
 */

#[no_mangle]
pub extern "C" fn name() {
    let contract = CasperCEP47Contract::new();
    ret(contract.name())
}

#[no_mangle]
pub extern "C" fn symbol() {
    let contract = CasperCEP47Contract::new();
    ret(contract.symbol())
}

#[no_mangle]
pub extern "C" fn uri() {
    let contract = CasperCEP47Contract::new();
    ret(contract.uri())
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: PublicKey = runtime::get_named_arg("account");
    let contract = CasperCEP47Contract::new();
    ret(contract.balance_of(account))
}

#[no_mangle]
pub extern "C" fn owner_of() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let contract = CasperCEP47Contract::new();
    ret(contract.owner_of(token_id))
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let contract = CasperCEP47Contract::new();
    ret(contract.total_supply())
}

#[no_mangle]
pub extern "C" fn token_uri() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let contract = CasperCEP47Contract::new();
    ret(contract.token_uri(token_id))
}

#[no_mangle]
pub extern "C" fn tokens() {
    let owner: PublicKey = runtime::get_named_arg("owner");
    let contract = CasperCEP47Contract::new();
    ret(contract.tokens(owner))
}

#[no_mangle]
pub extern "C" fn mint_one() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_uri: URI = runtime::get_named_arg("token_uri");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_one(recipient, token_uri);
}

#[no_mangle]
pub extern "C" fn mint_many() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_uris: Vec<URI> = runtime::get_named_arg("token_uris");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_many(recipient, token_uris);
}

#[no_mangle]
pub extern "C" fn mint_copies() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_uri: URI = runtime::get_named_arg("token_uri");
    let count: U256 = runtime::get_named_arg("count");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_copies(recipient, token_uri, count);
}

#[no_mangle]
pub extern "C" fn transfer_token() {
    let sender: PublicKey = runtime::get_named_arg("sender");
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let mut contract = CasperCEP47Contract::new();
    contract.transfer_token(sender, recipient, token_id);
}

#[no_mangle]
pub extern "C" fn transfer_many_tokens() {
    let sender: PublicKey = runtime::get_named_arg("sender");
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let mut contract = CasperCEP47Contract::new();
    contract.transfer_many_tokens(sender, recipient, token_ids);
}

#[no_mangle]
pub extern "C" fn transfer_all_tokens() {
    let sender: PublicKey = runtime::get_named_arg("sender");
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let mut contract = CasperCEP47Contract::new();
    contract.transfer_all_tokens(sender, recipient);
}

pub fn get_entrypoints(package_hash: Option<ContractPackageHash>) -> EntryPoints {
    let secure = if let Some(contract_package_hash) = package_hash {
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
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("uri", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256, None));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", CLType::PublicKey)],
        CLType::U256,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "owner_of",
        vec![Parameter::new("token_id", CLType::String)],
        CLType::Option(Box::new(CLType::PublicKey)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "token_uri",
        vec![Parameter::new("token_id", CLType::String)],
        CLType::Option(Box::new(CLType::String)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "tokens",
        vec![Parameter::new("owner", CLType::PublicKey)],
        CLType::List(Box::new(CLType::String)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "total_supply",
        vec![Parameter::new("owner", CLType::PublicKey)],
        CLType::List(Box::new(CLType::String)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "mint_one",
        vec![
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_uri", CLType::String),
        ],
        CLType::Unit,
        if secure {
            Some("deployer_access")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_many",
        vec![
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_uris", CLType::List(Box::new(CLType::String))),
        ],
        CLType::Unit,
        if secure {
            Some("deployer_access")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_copies",
        vec![
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_uri", CLType::String),
            Parameter::new("count", CLType::U256),
        ],
        CLType::Unit,
        if secure {
            Some("deployer_access")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_token",
        vec![
            Parameter::new("sender", CLType::PublicKey),
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_id", CLType::String),
        ],
        CLType::Unit,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_many_tokens",
        vec![
            Parameter::new("sender", CLType::PublicKey),
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_ids", CLType::List(Box::new(CLType::String))),
        ],
        CLType::Unit,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_all_tokens",
        vec![
            Parameter::new("sender", CLType::PublicKey),
            Parameter::new("recipient", CLType::PublicKey),
        ],
        CLType::Unit,
        None,
    ));
    entry_points
}

pub fn deploy(
    token_name: &str,
    token_symbol: &str,
    token_uri: &str,
    entry_points: EntryPoints,
    contract_package_hash: ContractPackageHash,
) {
    let mut named_keys = NamedKeys::new();
    named_keys.insert("name".to_string(), storage::new_uref(token_name).into());
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    named_keys.insert("uri".to_string(), storage::new_uref(token_uri).into());
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(U256::zero()).into(),
    );

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key("caspercep47_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("caspercep47_contract_hash", contract_hash_pack.into());
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

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

fn owner_key(token_id: &TokenId) -> String {
    format!("owners_{}", token_id)
}

fn uri_key(token_id: &TokenId) -> String {
    format!("uris_{}", token_id)
}

fn token_key(account: &AccountHash) -> String {
    format!("tokens_{}", account)
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
