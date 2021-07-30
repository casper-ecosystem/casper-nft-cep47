use casper_types::{AccessRights, ContractPackageHash};
use rand::Rng;

use crate::{
    AsymmetricType, CEP47Contract, CEP47Storage, Error, Key, Meta, TokenId, URef, WithStorage, U256,
};
use casper_types::PublicKey;
use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    sync::Mutex,
};

struct TestStorage {
    name: String,
    symbol: String,
    meta: Meta,
    paused: bool,
    total_supply: U256,
    tokens: BTreeMap<Key, Vec<TokenId>>,
    token_metas: BTreeMap<TokenId, Meta>,
    balances: BTreeMap<Key, U256>,
    belongs_to: BTreeMap<TokenId, Key>,
    urefs: BTreeMap<URef, TokenId>,
    token_id_generator: u32,
}

impl TestStorage {
    pub fn new() -> TestStorage {
        TestStorage {
            name: String::from("Casper Enhancement Proposal 47"),
            symbol: String::from("CEP47"),
            meta: meta::contract_info(),
            paused: false,
            total_supply: U256::from(0),
            tokens: BTreeMap::new(),
            balances: BTreeMap::new(),
            belongs_to: BTreeMap::new(),
            token_metas: BTreeMap::new(),
            urefs: BTreeMap::new(),
            token_id_generator: 1,
        }
    }
}

impl CEP47Storage for TestStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }

    fn meta(&self) -> Meta {
        self.meta.clone()
    }

    fn balance_of(&self, owner: &Key) -> U256 {
        let owner_balance = self.balances.get(owner);
        owner_balance.cloned().unwrap_or_default()
    }

    fn owner_of(&self, token_id: &TokenId) -> Option<Key> {
        let owner = self.belongs_to.get(token_id);
        owner.cloned()
    }

    fn total_supply(&self) -> U256 {
        self.total_supply
    }

    fn token_meta(&self, token_id: &TokenId) -> Option<Meta> {
        let meta = self.token_metas.get(token_id);
        meta.cloned()
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn unpause(&mut self) {
        self.paused = false;
    }

    fn get_tokens(&self, owner: &Key) -> Vec<TokenId> {
        let owner_tokens = self.tokens.get(owner);
        owner_tokens.cloned().unwrap_or_default()
    }

    fn set_tokens(&mut self, owner: &Key, token_ids: Vec<TokenId>) {
        let owner_new_balance = U256::from(token_ids.len() as u64);

        let owner_tokens = self.get_tokens(owner);
        for token_id in owner_tokens.clone() {
            self.belongs_to.remove(&token_id);
        }
        for token_id in token_ids.clone() {
            self.belongs_to.insert(token_id, *owner);
        }

        self.tokens.insert(*owner, token_ids);
        self.balances.insert(*owner, owner_new_balance);
    }

    fn mint_many(&mut self, recipient: &Key, token_ids: &Vec<TokenId>, token_metas: &Vec<Meta>) {
        let recipient_balance = self.balances.get(recipient);
        let recipient_tokens = self.tokens.get(recipient);

        let mut recipient_new_balance = recipient_balance.copied().unwrap_or_default();
        let mut recipient_new_tokens = recipient_tokens.cloned().unwrap_or_default();

        for (token_id, token_meta) in token_ids.iter().zip(token_metas) {
            self.token_metas
                .insert(token_id.clone(), token_meta.clone());
            recipient_new_tokens.push(token_id.clone());
            self.belongs_to.insert(token_id.clone(), *recipient);
            recipient_new_balance = recipient_new_balance + 1;
            self.total_supply = self.total_supply + 1;
        }
        self.balances.insert(*recipient, recipient_new_balance);
        self.tokens.insert(*recipient, recipient_new_tokens);
    }

    fn burn_many(&mut self, owner: &Key, token_ids: &Vec<TokenId>) {
        let owner_tokens = self.tokens.get(owner);
        let owner_balance = self.balances.get(owner);

        let mut owner_new_balance = owner_balance.copied().unwrap_or_default();
        let mut owner_new_tokens = owner_tokens.cloned().unwrap_or_default();

        for token_id in token_ids.clone() {
            let index = owner_new_tokens
                .iter()
                .position(|x| *x == token_id.clone())
                .unwrap();
            owner_new_tokens.remove(index);
            self.token_metas.remove(&token_id.clone());
            self.belongs_to.remove(&token_id.clone());
            owner_new_balance = owner_new_balance - 1;
            self.total_supply = self.total_supply - 1;
        }
        self.balances.insert(*owner, owner_new_balance);
        self.tokens.insert(*owner, owner_new_tokens);
    }

    fn update_token_metadata(&mut self, token_id: &TokenId, meta: Meta) {
        self.token_metas.insert(token_id.clone(), meta).unwrap();
    }

    fn gen_token_ids(&mut self, n: u32) -> Vec<TokenId> {
        let mut tokens = Vec::new();
        for _ in 0..n {
            let id = format!("token_{}", &self.token_id_generator);
            tokens.push(id);
            self.token_id_generator += 1;
        }
        tokens
    }

    fn validate_token_ids(&self, token_ids: &Vec<TokenId>) -> bool {
        for token_id in token_ids {
            if self.owner_of(token_id).is_some() {
                return false;
            }
        }
        true
    }

    fn emit(&mut self, _event: crate::events::CEP47Event) {}

    fn contact_package_hash(&self) -> casper_types::ContractPackageHash {
        [1u8; 32].into()
    }
}

struct TestContract {
    storage: TestStorage,
}

impl TestContract {
    pub fn new() -> TestContract {
        TestContract {
            storage: TestStorage::new(),
        }
    }
}

impl WithStorage<TestStorage> for TestContract {
    fn storage(&self) -> &TestStorage {
        &self.storage
    }

    fn storage_mut(&mut self) -> &mut TestStorage {
        &mut self.storage
    }
}

impl CEP47Contract<TestStorage> for TestContract {}

mod meta {
    use super::BTreeMap;

    pub fn contract_info() -> BTreeMap<String, String> {
        btreemap! {
            "github".to_string() => "https://github.com/casper-ecosystem/casper-nft-cep47".to_string()
        }
    }

    pub fn apple() -> BTreeMap<String, String> {
        btreemap! {
            "fruit".to_string() => "Apple".to_string(),
            "size".to_string() => "A".to_string()
        }
    }

    pub fn orange() -> BTreeMap<String, String> {
        btreemap! {
            "fruit".to_string() => "Orange".to_string(),
            "size".to_string() => "B".to_string()
        }
    }

    pub fn banana() -> BTreeMap<String, String> {
        btreemap! {
            "fruit".to_string() => "Banana".to_string(),
            "size".to_string() => "B".to_string()
        }
    }
}

#[test]
fn test_metadata() {
    let contract = TestContract::new();
    assert_eq!(
        contract.name(),
        String::from("Casper Enhancement Proposal 47")
    );
    assert_eq!(contract.symbol(), String::from("CEP47"));
    assert_eq!(contract.meta(), meta::contract_info());
}

#[test]
fn test_mint_many() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();
    let bob_first_token_id = "banana_01".to_string();
    let bob_second_token_id = "orange_02".to_string();
    let bob_token_ids = vec![bob_first_token_id.clone(), bob_second_token_id];

    assert_eq!(contract.total_supply(), U256::from(0));
    let _: Result<(), Error> = contract.mint_many(&ali, None, vec![meta::apple()]);
    let _: Result<(), Error> = contract.mint_many(
        &bob,
        Some(bob_token_ids.clone()),
        vec![meta::banana(), meta::orange()],
    );
    assert_eq!(contract.total_supply(), U256::from(3));

    let ali_balance = contract.balance_of(&ali);
    assert_eq!(ali_balance, U256::from(1));
    let bob_balance = contract.balance_of(&bob);
    assert_eq!(bob_balance, U256::from(2));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let ali_first_token_meta: Meta = contract.token_meta(ali_tokens.get(0).unwrap()).unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());

    let bob_tokens: Vec<TokenId> = contract.tokens(&bob);
    assert_eq!(bob_token_ids, bob_tokens);
    let bob_first_token_meta: Meta = contract.token_meta(&bob_first_token_id).unwrap();
    assert_eq!(bob_first_token_meta, meta::banana());
}
#[test]
fn test_mint_copies() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));
    let _: Result<(), Error> = contract.mint_copies(&ali, None, meta::apple(), 7);
    assert_eq!(contract.total_supply(), U256::from(7));

    let ali_balance = contract.balance_of(&ali);
    assert_eq!(ali_balance, U256::from(7));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let ali_first_token_meta: Meta = contract.token_meta(ali_tokens.get(0).unwrap()).unwrap();
    let ali_third_token_meta: Meta = contract.token_meta(ali_tokens.get(2).unwrap()).unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());
    assert_eq!(ali_third_token_meta, meta::apple());
}
#[test]
fn test_burn_many() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));

    let _: Result<(), Error> = contract.mint_many(
        &ali,
        None,
        vec![meta::banana(), meta::orange(), meta::apple()],
    );
    assert_eq!(contract.total_supply(), U256::from(3));

    let ali_balance = contract.balance_of(&ali);
    assert_eq!(ali_balance, U256::from(3));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let banana = ali_tokens.get(0).unwrap().clone();
    let orange = ali_tokens.get(1).unwrap().clone();
    let apple = ali_tokens.get(2).unwrap().clone();

    contract.burn_many(&ali, vec![banana.clone(), apple.clone()]);
    let ali_tokens_after_burn = contract.tokens(&ali);
    assert_eq!(ali_tokens_after_burn, vec![orange.clone()]);

    assert!(contract.token_meta(&banana).is_none());
    assert!(contract.token_meta(&orange).is_some());
    assert!(contract.token_meta(&apple).is_none());

    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(&ali), U256::from(1));
}
#[test]
fn test_burn_one() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));
    let _: Result<(), Error> = contract.mint_many(&ali, None, vec![meta::banana(), meta::orange()]);
    assert_eq!(contract.total_supply(), U256::from(2));

    let mut ali_balance = contract.balance_of(&ali);
    assert_eq!(ali_balance, U256::from(2));

    let mut ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    contract.burn_one(&ali, ali_tokens.get(0).unwrap().clone());
    let mut ali_first_token_meta = contract.token_meta(ali_tokens.get(0).unwrap());
    assert_eq!(ali_first_token_meta, None);

    ali_tokens = contract.tokens(&ali);
    ali_first_token_meta = contract.token_meta(ali_tokens.get(0).unwrap());
    assert_eq!(ali_first_token_meta, Some(meta::orange()));

    assert_eq!(contract.total_supply(), U256::from(1));
    ali_balance = contract.balance_of(&ali);
    assert_eq!(ali_balance, U256::from(1));
}
#[test]
fn test_transfer_token() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));
    let _: Result<(), Error> = contract.mint_one(&ali, None, meta::apple());
    assert_eq!(contract.total_supply(), U256::from(1));

    let mut ali_balance = contract.balance_of(&ali);
    let mut bob_balance = contract.balance_of(&bob);
    assert_eq!(ali_balance, U256::from(1));
    assert_eq!(bob_balance, U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let ali_first_token_id: TokenId = ali_tokens.get(0).unwrap().clone();
    let ali_first_token_meta: Meta = contract.token_meta(&ali_first_token_id).unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());

    let transfer_res = contract.transfer_token(&ali, &bob, &ali_first_token_id);
    assert!(transfer_res.is_ok());
    ali_balance = contract.balance_of(&ali);
    bob_balance = contract.balance_of(&bob);
    assert_eq!(ali_balance, U256::from(0));
    assert_eq!(bob_balance, U256::from(1));

    let owner_of_first_token_id = contract.owner_of(&ali_first_token_id);
    assert_eq!(owner_of_first_token_id.unwrap(), bob);
}
#[test]
fn test_transfer_all_tokens() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));
    let _: Result<(), Error> = contract.mint_many(&ali, None, vec![meta::apple(), meta::banana()]);
    let _: Result<(), Error> = contract.mint_one(&ali, None, meta::apple());
    assert_eq!(contract.total_supply(), U256::from(3));

    let mut ali_balance = contract.balance_of(&ali);
    let mut bob_balance = contract.balance_of(&bob);
    assert_eq!(ali_balance, U256::from(3));
    assert_eq!(bob_balance, U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
    let ali_second_token_meta: Meta = contract.token_meta(&ali_second_token_id).unwrap();
    assert_eq!(ali_second_token_meta, meta::banana());

    let transfer_res = contract.transfer_all_tokens(&ali, &bob);
    assert!(transfer_res.is_ok());

    ali_balance = contract.balance_of(&ali);
    bob_balance = contract.balance_of(&bob);
    assert_eq!(ali_balance, U256::from(0));
    assert_eq!(bob_balance, U256::from(3));

    let owner_of_second_token_id = contract.owner_of(&ali_second_token_id);
    assert_eq!(owner_of_second_token_id.unwrap(), bob);
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();

    assert_eq!(contract.total_supply(), U256::from(0));
    let _ = contract.mint_many(&ali, None, vec![meta::apple(), meta::banana()]);
    let _: Result<(), Error> = contract.mint_copies(&ali, None, meta::apple(), 3);
    assert_eq!(contract.total_supply(), U256::from(5));

    assert_eq!(contract.balance_of(&ali), U256::from(5));
    assert_eq!(contract.balance_of(&bob), U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(&ali);
    let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
    let ali_second_token_meta: Meta = contract.token_meta(&ali_second_token_id).unwrap();
    let ali_third_token_id: TokenId = ali_tokens.get(2).unwrap().clone();
    let ali_third_token_meta: Meta = contract.token_meta(&ali_third_token_id).unwrap();
    assert_eq!(ali_second_token_meta, meta::banana());
    assert_eq!(ali_third_token_meta, meta::apple());

    let transfer_res = contract.transfer_many_tokens(
        &ali,
        &bob,
        &vec![ali_second_token_id.clone(), ali_third_token_id.clone()],
    );
    assert!(transfer_res.is_ok());
    assert_eq!(contract.balance_of(&ali), U256::from(3));
    assert_eq!(contract.balance_of(&bob), U256::from(2));

    let owner_of_second_token_id = contract.owner_of(&ali_second_token_id);
    let owner_of_third_token_id = contract.owner_of(&ali_third_token_id);
    assert_eq!(owner_of_second_token_id.unwrap(), bob);
    assert_eq!(owner_of_third_token_id.unwrap(), bob);
}

#[test]
fn test_update_metadata() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();

    let token_id = TokenId::from("new_token");
    let _: Result<(), Error> = contract.mint_one(&ali, Some(token_id.clone()), meta::apple());
    assert_eq!(meta::apple(), contract.token_meta(&token_id).unwrap());
    let _: Result<(), Error> = contract.update_token_metadata(token_id.clone(), meta::banana());
    assert_eq!(meta::banana(), contract.token_meta(&token_id).unwrap());
}
