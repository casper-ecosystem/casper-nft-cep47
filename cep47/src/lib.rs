#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    AccessRights, ApiError, AsymmetricType, CLType, CLTyped, CLValue, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, HashAddr, Key, Parameter, PublicKey,
    URef, U256,
};
pub use cep47_logic::Meta;
use cep47_logic::{CEP47Contract, CEP47Storage, TokenId, WithStorage};

use core::convert::TryInto;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::AddAssign,
};

pub struct CasperCEP47Storage {}
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

    fn meta(&self) -> Meta {
        get_key::<Meta>("meta").unwrap()
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

    fn onwer_of(&self, token_id: &TokenId) -> Option<PublicKey> {
        get_key::<PublicKey>(&owner_key(token_id))
    }

    fn total_supply(&self) -> U256 {
        get_key::<U256>("total_supply").unwrap()
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        get_key::<Meta>(&meta_key(&token_id))
    }

    fn is_paused(&self) -> bool {
        get_key::<bool>("paused").unwrap()
    }

    fn pause(&mut self) {
        set_key("paused", true);
    }

    fn unpause(&mut self) {
        set_key("paused", false);
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
        let owner_prev_balance = self.balance_of(owner.clone());
        let owner_new_balance = U256::from(token_ids.len() as u64);
        let prev_total_supply = self.total_supply();

        let owner_tokens = self.get_tokens(owner.clone());
        for token_id in owner_tokens.clone() {
            remove_key(&owner_key(&token_id));
        }
        for token_id in token_ids.clone() {
            set_key(&owner_key(&token_id), owner.clone());
        }
        set_key(&token_key(&owner.to_account_hash()), token_ids);
        set_key(&balance_key(&owner.to_account_hash()), owner_new_balance);
        set_key(
            "total_supply",
            prev_total_supply - owner_prev_balance + owner_new_balance,
        );
    }

    fn mint_many(&mut self, recipient: PublicKey, token_ids: Vec<TokenId>, token_metas: Vec<Meta>) {
        let mut recipient_tokens = self.get_tokens(recipient.clone());
        let mut recipient_balance = self.balance_of(recipient.clone());
        let mut total_supply = self.total_supply();

        for (token_id, token_meta) in token_ids.iter().zip(token_metas) {
            recipient_tokens.push(token_id.clone());
            set_key(&meta_key(&token_id), token_meta);
            set_key(&owner_key(&token_id), recipient.clone());

            // Emit event.
            emit_mint_one_event(&recipient, &token_id);
        }

        set_key(&token_key(&recipient.to_account_hash()), recipient_tokens);

        let new_tokens_count = U256::from(token_ids.len() as u32); 
        recipient_balance = recipient_balance + new_tokens_count;
        set_key(
            &balance_key(&recipient.to_account_hash()),
            recipient_balance,
        );
        total_supply = total_supply + new_tokens_count;
        set_key("total_supply", total_supply);
    }

    fn burn_many(&mut self, owner: PublicKey, token_ids: Vec<TokenId>) {
        let mut owner_tokens = self.get_tokens(owner.clone());
        let mut owner_balance = self.balance_of(owner.clone());
        let mut total_supply = self.total_supply();

        for token_id in token_ids.clone() {
            let index = owner_tokens
                .iter()
                .position(|x| *x == token_id.clone())
                .unwrap_or_revert();
            owner_tokens.remove(index);
            remove_key(&meta_key(&token_id));
            remove_key(&owner_key(&token_id));
            owner_balance = owner_balance - 1;
            total_supply = total_supply - 1;

            emit_burn_one_event(&owner, &token_id);
        }
        set_key(&balance_key(&owner.to_account_hash()), owner_balance);
        set_key(&token_key(&owner.to_account_hash()), owner_tokens);
        set_key("total_supply", total_supply);
    }

    fn burn_one(&mut self, owner: PublicKey, token_id: TokenId) {
        let mut owner_tokens = self.get_tokens(owner.clone());
        let owner_balance = self.balance_of(owner.clone());
        let total_supply = self.total_supply();
        let index = owner_tokens
            .iter()
            .position(|x| *x == token_id.clone())
            .unwrap_or_revert();
        owner_tokens.remove(index);
        remove_key(&meta_key(&token_id));
        remove_key(&owner_key(&token_id));
        set_key(&balance_key(&owner.to_account_hash()), owner_balance - 1);
        set_key(&token_key(&owner.to_account_hash()), owner_tokens);
        set_key("total_supply", total_supply - 1);
        emit_burn_one_event(&owner, &token_id);
    }

    fn update_token_metadata(&mut self, token_id: TokenId, meta: Meta) {
        set_key(&meta_key(&token_id), meta);
    }

    fn gen_token_ids(&mut self, n: u32) -> Vec<TokenId> {
        let block_time = runtime::get_blocktime();
        let mut token_ids = Vec::new();
        let nonce = get_nonce();
        for i in nonce..nonce + n {
            let mut bytes: Vec<u8> = block_time.to_bytes().unwrap_or_revert();
            bytes.append(&mut i.to_bytes().unwrap_or_revert());            
            let hash = runtime::blake2b(bytes);
            token_ids.push(hex::encode(hash));
        };
        set_nonce(nonce+n);
        token_ids
    }

    fn validate_token_ids(&self, token_ids: &Vec<TokenId>) -> bool {
       for token_id in token_ids {
           if self.onwer_of(token_id).is_some() {
               return false;
           }
       }
       true
    }
}

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
    ret(contract.symbol())
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
    let account: PublicKey = runtime::get_named_arg("account");
    let contract = CasperCEP47Contract::new();
    ret(contract.balance_of(account))
}

#[cfg(not(feature = "no_owner_of"))]
#[no_mangle]
pub extern "C" fn owner_of() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let contract = CasperCEP47Contract::new();
    ret(contract.owner_of(token_id))
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
    ret(contract.token_meta(token_id))
}

#[cfg(not(feature = "no_tokens"))]
#[no_mangle]
pub extern "C" fn tokens() {
    let owner: PublicKey = runtime::get_named_arg("owner");
    let contract = CasperCEP47Contract::new();
    ret(contract.tokens(owner))
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
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_id: Option<TokenId> = runtime::get_named_arg("token_id");
    let token_meta: Meta = runtime::get_named_arg("token_meta");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_one(recipient, token_id, token_meta).unwrap_or_revert();
}

#[cfg(not(feature = "no_mint_many"))]
#[no_mangle]
pub extern "C" fn mint_many() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_ids: Option<Vec<TokenId>> = runtime::get_named_arg("token_ids");
    let token_metas: Vec<Meta> = runtime::get_named_arg("token_metas");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_many(recipient, token_ids, token_metas).unwrap_or_revert();
}

#[cfg(not(feature = "no_mint_copies"))]
#[no_mangle]
pub extern "C" fn mint_copies() {
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_ids: Option<Vec<TokenId>> = runtime::get_named_arg("token_ids");
    let token_meta: Meta = runtime::get_named_arg("token_meta");
    let count: u32 = runtime::get_named_arg("count");
    let mut contract = CasperCEP47Contract::new();
    contract.mint_copies(recipient, token_ids, token_meta, count).unwrap_or_revert();
}

#[cfg(not(feature = "no_burn_many"))]
#[no_mangle]
pub extern "C" fn burn_many() {
    let owner: PublicKey = runtime::get_named_arg("owner");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_many(owner, token_ids);
}

#[cfg(not(feature = "no_burn_one"))]
#[no_mangle]
pub extern "C" fn burn_one() {
    let owner: PublicKey = runtime::get_named_arg("owner");
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let mut contract = CasperCEP47Contract::new();
    contract.burn_one(owner, token_id);
}

#[cfg(not(feature = "no_transfer_token"))]
#[no_mangle]
pub extern "C" fn transfer_token() {
    let caller: AccountHash = runtime::get_caller();
    let sender: PublicKey = runtime::get_named_arg("sender");
    if sender.to_account_hash() != caller {
        runtime::revert(ApiError::PermissionDenied);
    }
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_id: TokenId = runtime::get_named_arg("token_id");

    emit_transfer_token_event(&sender, &recipient, &token_id);

    let mut contract = CasperCEP47Contract::new();
    let res = contract.transfer_token(sender, recipient, token_id);
    res.unwrap_or_revert();
}

#[cfg(not(feature = "no_transfer_many_tokens"))]
#[no_mangle]
pub extern "C" fn transfer_many_tokens() {
    let caller: AccountHash = runtime::get_caller();
    let sender: PublicKey = runtime::get_named_arg("sender");
    if sender.to_account_hash() != caller {
        runtime::revert(ApiError::PermissionDenied);
    }
    let recipient: PublicKey = runtime::get_named_arg("recipient");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");

    for token_id in &token_ids {
        emit_transfer_token_event(&sender, &recipient, &token_id);
    }

    let mut contract = CasperCEP47Contract::new();
    let res = contract.transfer_many_tokens(sender, recipient, token_ids);
    res.unwrap_or_revert();
}

#[cfg(not(feature = "no_transfer_all_tokens"))]
#[no_mangle]
pub extern "C" fn transfer_all_tokens() {
    let caller: AccountHash = runtime::get_caller();
    let sender: PublicKey = runtime::get_named_arg("sender");
    if sender.to_account_hash() != caller {
        runtime::revert(ApiError::PermissionDenied);
    }
    let recipient: PublicKey = runtime::get_named_arg("recipient");

    emit_transfer_all_tokens_event(&sender, &recipient);

    let mut contract = CasperCEP47Contract::new();
    let res = contract.transfer_all_tokens(sender, recipient);
    res.unwrap_or_revert();
}

#[cfg(not(feature = "no_update_token_metadata"))]
#[no_mangle]
pub extern "C" fn update_token_metadata() {
    let token_id: TokenId = runtime::get_named_arg("token_id");
    let meta: Meta = runtime::get_named_arg("token_meta");
    let mut contract = CasperCEP47Contract::new();
    let res = contract.update_token_metadata(token_id, meta);
    res.unwrap_or_revert();
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
        runtime::put_key("deployer_group_access", Key::URef(deployer_group[0]));
        true
    } else {
        false
    };

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("meta", vec![], Meta::cl_type(), None));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256, None));
    entry_points.add_entry_point(endpoint("is_paused", vec![], CLType::Bool, None));
    entry_points.add_entry_point(endpoint(
        "pause",
        vec![],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "unpause",
        vec![],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", CLType::PublicKey)],
        CLType::U256,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::PublicKey)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
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
            Parameter::new("token_ids", CLType::Option(Box::new(TokenId::cl_type()))),
            Parameter::new("token_meta", Meta::cl_type()), 
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_many",
        vec![
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_ids", CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type()))))),
            Parameter::new("token_metas", CLType::List(Box::new(Meta::cl_type()))),
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_copies",
        vec![
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_ids", CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type()))))),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "update_token_metadata",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "burn_one",
        vec![
            Parameter::new("owner", CLType::PublicKey),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "burn_many",
        vec![
            Parameter::new("owner", CLType::PublicKey),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        CLType::Unit,
        if secure {
            Some("deployer")
        } else {
            None
        },
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_token",
        vec![
            Parameter::new("sender", CLType::PublicKey),
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Unit,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_many_tokens",
        vec![
            Parameter::new("sender", CLType::PublicKey),
            Parameter::new("recipient", CLType::PublicKey),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
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
    token_name: String,
    token_symbol: String,
    token_meta: Meta,
    entry_points: EntryPoints,
    contract_package_hash: ContractPackageHash,
    paused: bool,
) {
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "name".to_string(),
        storage::new_uref(token_name.clone()).into(),
    );
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    named_keys.insert("meta".to_string(), storage::new_uref(token_meta).into());
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(U256::zero()).into(),
    );
    named_keys.insert("paused".to_string(), storage::new_uref(paused).into());
    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(
        format!("{}_contract", &token_name).as_str(),
        contract_hash.into(),
    );
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key(
        format!("{}_contract_hash", token_name).as_str(),
        contract_hash_pack.into(),
    );
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

fn meta_key(token_id: &TokenId) -> String {
    format!("metas_{}", token_id)
}

fn token_key(account: &AccountHash) -> String {
    format!("tokens_{}", account)
}

pub fn endpoint(
    name: &str,
    param: Vec<Parameter>,
    ret: CLType,
    access: Option<&str>,
) -> EntryPoint {
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

pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

pub fn emit_mint_one_event(recipient: &PublicKey, token_id: &TokenId) {
    let mut event = BTreeMap::new();
    event.insert("contract_package_hash", package_hash().to_string());
    event.insert("event_type", "cep47_mint_one".to_string());
    event.insert("recipient", recipient.to_string());
    event.insert("token_id", token_id.to_string());
    emit_event(event);
}

pub fn emit_burn_one_event(owner: &PublicKey, token_id: &TokenId) {
    let mut event = BTreeMap::new();
    event.insert("contract_package_hash", package_hash().to_string());
    event.insert("event_type", "cep47_burn_one".to_string());
    event.insert("owner", owner.to_string());
    event.insert("token_id", token_id.to_string());
    emit_event(event);
}

pub fn emit_transfer_token_event(sender: &PublicKey, recipient: &PublicKey, token_id: &TokenId) {
    let mut event = BTreeMap::new();
    event.insert("contract_package_hash", package_hash().to_string());
    event.insert("event_type", "cep47_transfer_token".to_string());
    event.insert("sender", sender.to_string());
    event.insert("recipient", recipient.to_string());
    event.insert("token_id", token_id.to_string());
    emit_event(event);
}

pub fn emit_transfer_all_tokens_event(sender: &PublicKey, recipient: &PublicKey) {
    let mut event = BTreeMap::new();
    event.insert("contract_package_hash", package_hash().to_string());
    event.insert("event_type", "cep47_transfer_all_tokens".to_string());
    event.insert("sender", sender.to_string());
    event.insert("recipient", recipient.to_string());
    emit_event(event);
}

pub fn emit_event(event: BTreeMap<&str, String>) {
    let _: URef = storage::new_uref(event);
}

pub fn package_hash() -> ContractPackageHash {
    let key: [u8; 32] = get_key("contract_package_hash").unwrap_or_revert();
    key.into()
}

fn get_nonce() -> u32 {
    get_key("nonce").unwrap_or_default()
}

fn set_nonce(nonce: u32) {
    set_key("nonce", nonce);
}
