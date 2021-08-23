use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, Dict};

use crate::{event::CEP47Event, Meta, TokenId};

const BALANCES_DICT: &str = "balances";
pub const ALLOWANCES_DICT: &str = "allowances";
const METADATA_DICT: &str = "metadata";
const OWNERS_DICT: &str = "owners";
const OWNED_TOKENS_BY_INDEX_DICT: &str = "owned_tokens_by_index";
const OWNED_INDEXES_BY_TOKEN_DICT: &str = "owned_indexes_by_token";
const OWNED_TOKENS_LENGTH_DICT: &str = "owned_tokens_length";
const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub const NAME: &str = "name";
pub const META: &str = "meta";
pub const SYMBOL: &str = "symbol";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const NONCE: &str = "nonce";

pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Owners {
    dict: Dict,
}

impl Owners {
    pub fn instance() -> Owners {
        Owners {
            dict: Dict::instance(OWNERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(OWNERS_DICT)
    }

    pub fn get(&self, key: &str) -> Option<Key> {
        self.dict.get(key)
    }

    pub fn set(&self, key: &str, value: Key) {
        self.dict.set(key, value);
    }

    pub fn remove(&self, key: &str) {
        self.dict.remove::<Key>(key);
    }
}

pub struct Metadata {
    dict: Dict,
}

impl Metadata {
    pub fn instance() -> Metadata {
        Metadata {
            dict: Dict::instance(METADATA_DICT),
        }
    }

    pub fn init() {
        Dict::init(METADATA_DICT)
    }

    pub fn get(&self, key: &str) -> Option<Meta> {
        self.dict.get(key)
    }

    pub fn set(&self, key: &str, value: Meta) {
        self.dict.set(key, value);
    }

    pub fn remove(&self, key: &str) {
        self.dict.remove::<Meta>(key);
    }
}

pub struct OwnedTokens {
    token_dict: Dict,
    index_dict: Dict,
    length_dict: Dict,
}

impl OwnedTokens {
    pub fn instance() -> OwnedTokens {
        OwnedTokens {
            token_dict: Dict::instance(OWNED_TOKENS_BY_INDEX_DICT),
            index_dict: Dict::instance(OWNED_INDEXES_BY_TOKEN_DICT),
            length_dict: Dict::instance(OWNED_TOKENS_LENGTH_DICT),
        }
    }

    pub fn init() {
        Dict::init(OWNED_TOKENS_BY_INDEX_DICT);
        Dict::init(OWNED_INDEXES_BY_TOKEN_DICT);
        Dict::init(OWNED_TOKENS_LENGTH_DICT);
    }

    pub fn get_token_by_index(&self, owner: &Key, index: &u32) -> Option<TokenId> {
        self.token_dict.get(&key_and_value_to_str(owner, index))
    }

    pub fn get_index_by_token(&self, owner: &Key, value: &str) -> Option<u32> {
        self.index_dict
            .get(&key_and_value_to_str(owner, &value.to_string()))
    }

    pub fn get_tokens_len(&self, owner: &Key) -> Option<u32> {
        self.length_dict.get(&key_to_str(owner))
    }

    pub fn set_tokens_len(&self, owner: &Key, value: u32) {
        self.length_dict.set(&key_to_str(owner), value);
    }

    pub fn set_token(&self, owner: &Key, value: TokenId) {
        let length = self.get_tokens_len(owner).unwrap_or_default();
        self.index_dict
            .set(&key_and_value_to_str(owner, &value), length);
        self.token_dict
            .set(&key_and_value_to_str(owner, &length), value);
        self.set_tokens_len(owner, length + 1);
    }

    pub fn remove_token(&self, owner: &Key, value: TokenId) {
        let length = self.get_tokens_len(owner).unwrap_or_revert();
        let index = self.get_index_by_token(owner, &value).unwrap_or_revert();
        match length.cmp(&(index + 1)) {
            core::cmp::Ordering::Equal => {
                self.token_dict
                    .remove::<TokenId>(&key_and_value_to_str(owner, &(length - 1)));
                self.set_tokens_len(owner, length - 1);
            }
            core::cmp::Ordering::Greater => {
                let last = self.get_token_by_index(owner, &(length - 1));
                self.index_dict.set(
                    &key_and_value_to_str(owner, &last.clone().unwrap_or_revert()),
                    index,
                );
                self.token_dict.set(
                    &key_and_value_to_str(owner, &index),
                    last.unwrap_or_revert(),
                );
                self.token_dict
                    .remove::<TokenId>(&key_and_value_to_str(owner, &(length - 1)));
                self.set_tokens_len(owner, length - 1);
            }
            core::cmp::Ordering::Less => {}
        }
        self.index_dict
            .remove::<u32>(&key_and_value_to_str(owner, &value));
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, token_id: &str) -> Option<Key> {
        self.dict
            .get(key_and_value_to_str::<TokenId>(owner, &token_id.to_string()).as_str())
    }

    pub fn set(&self, owner: &Key, token_id: &str, value: Key) {
        self.dict.set(
            key_and_value_to_str::<TokenId>(owner, &token_id.to_string()).as_str(),
            value,
        );
    }

    pub fn remove(&self, owner: &Key, token_id: &str) {
        self.dict
            .remove::<Key>(key_and_value_to_str::<TokenId>(owner, &token_id.to_string()).as_str());
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn meta() -> Meta {
    get_key(META).unwrap_or_revert()
}

pub fn set_meta(meta: Meta) {
    set_key(META, meta);
}

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn nonce() -> u32 {
    get_key(NONCE).unwrap_or_default()
}

pub fn set_nonce(nonce: u32) {
    set_key(NONCE, nonce);
}

pub fn contract_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn emit(event: &CEP47Event) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        CEP47Event::Mint {
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "cep47_mint_one".to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        CEP47Event::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "cep47_burn_one".to_string());
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        CEP47Event::Approve {
            owner,
            spender,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "cep47_approve_token".to_string());
                param.insert("owner", owner.to_string());
                param.insert("spender", spender.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        CEP47Event::Transfer {
            sender,
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "cep47_transfer_token".to_string());
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        CEP47Event::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "cep47_metadata_update".to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
