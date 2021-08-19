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

    fn mint_many(&mut self, recipient: &Key, token_ids: &Vec<TokenId>, token_metas: &Vec<Meta>) {
        let amount = token_ids.len();

        // Update balance.
        let recipient_balance = self.balances.get(recipient).copied().unwrap_or_default();
        let recipient_new_balance = recipient_balance + amount;
        self.balances.insert(*recipient, recipient_new_balance);

        self.total_supply = self.total_supply + amount;

        // Mint tokens.
        for (token_id, token_meta) in token_ids.iter().zip(token_metas) {
            self.token_metas
                .insert(token_id.clone(), token_meta.clone());
            self.belongs_to.insert(token_id.clone(), *recipient);
        }
    }

    fn transfer_many(&mut self, sender: &Key, recipient: &Key, token_ids: &Vec<TokenId>) {
        let amount = token_ids.len();
        let sender_balance = self.balances.get(sender).copied().unwrap_or_default();
        let recipient_balance = self.balances.get(recipient).copied().unwrap_or_default();
        self.balances.insert(*sender, sender_balance - amount);
        self.balances.insert(*recipient, recipient_balance + amount);

        for token_id in token_ids.iter() {
            self.belongs_to.insert(token_id.clone(), *recipient);
        }
    }

    fn burn_many(&mut self, owner: &Key, token_ids: &Vec<TokenId>) {
        let amount = token_ids.len();

        // Update balance.
        let recipient_balance = self.balances.get(owner).copied().unwrap_or_default();
        let recipient_new_balance = recipient_balance - amount;
        self.balances.insert(*owner, recipient_new_balance);

        self.total_supply = self.total_supply - amount;

        for token_id in token_ids.iter() {
            self.belongs_to.remove(token_id);
            self.token_metas.remove(token_id);
        }
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
    let ali_first_token_id = "banana_01".to_string();
    let ali_second_token_id = "orange_01".to_string();
    let ali_token_ids = vec![ali_first_token_id.clone(), ali_second_token_id.clone()];

    let bob: Key = bob_pk.to_account_hash().into();
    let bob_first_token_id = "banana_02".to_string();
    let bob_second_token_id = "apple_02".to_string();
    let bob_token_ids = vec![bob_first_token_id.clone(), bob_second_token_id.clone()];

    assert_eq!(contract.total_supply(), U256::from(0));
    contract
        .mint_many(
            &ali,
            Some(ali_token_ids),
            vec![meta::banana(), meta::orange()],
        )
        .unwrap();
    contract
        .mint_many(
            &bob,
            Some(bob_token_ids),
            vec![meta::banana(), meta::apple()],
        )
        .unwrap();

    // Check total balance.
    assert_eq!(contract.total_supply(), U256::from(4));

    // Check balances
    assert_eq!(contract.balance_of(&ali), U256::from(2));
    assert_eq!(contract.balance_of(&bob), U256::from(2));

    // Check ownership
    assert_eq!(&contract.owner_of(&ali_first_token_id).unwrap(), &ali);
    assert_eq!(&contract.owner_of(&ali_second_token_id).unwrap(), &ali);
    assert_eq!(&contract.owner_of(&bob_first_token_id).unwrap(), &bob);
    assert_eq!(&contract.owner_of(&bob_second_token_id).unwrap(), &bob);

    // Check metas.
    assert_eq!(
        contract.token_meta(&ali_first_token_id).unwrap(),
        meta::banana()
    );
    assert_eq!(
        contract.token_meta(&ali_second_token_id).unwrap(),
        meta::orange()
    );
    assert_eq!(
        contract.token_meta(&bob_first_token_id).unwrap(),
        meta::banana()
    );
    assert_eq!(
        contract.token_meta(&bob_second_token_id).unwrap(),
        meta::apple()
    );
}

#[test]
fn test_mint_copies() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let token_ids: Vec<String> = vec!["a", "b", "c", "d", "e", "f", "g"]
        .into_iter()
        .map(String::from)
        .collect();

    contract
        .mint_copies(&ali, Some(token_ids.clone()), meta::apple(), 7)
        .unwrap();

    assert_eq!(contract.total_supply(), U256::from(7));
    assert_eq!(contract.balance_of(&ali), U256::from(7));

    for token_id in &token_ids {
        assert_eq!(contract.owner_of(token_id).unwrap(), ali);
        assert_eq!(contract.token_meta(token_id).unwrap(), meta::apple());
    }
}

#[test]
fn test_burn_many() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let tokens_to_burn = vec!["a".to_string(), "b".to_string()];
    let tokens_to_remain = vec!["c".to_string(), "d".to_string(), "e".to_string()];
    let token_ids: Vec<String> = tokens_to_burn
        .iter()
        .cloned()
        .chain(tokens_to_remain.iter().cloned())
        .collect();

    contract
        .mint_copies(&ali, Some(token_ids), meta::banana(), 5)
        .unwrap();

    contract.burn_many(&ali, tokens_to_burn.clone()).unwrap();

    // Check balances
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(&ali), U256::from(3));

    // Check burned tokens
    for token_id in &tokens_to_burn {
        assert!(contract.owner_of(token_id).is_none());
        assert!(contract.token_meta(token_id).is_none());
    }

    // Check rest of tokens.
    for token_id in &tokens_to_remain {
        assert_eq!(&contract.owner_of(token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(token_id).unwrap(), &meta::banana());
    }
}

#[test]
fn test_burn_one() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let token_to_burn = "a".to_string();
    let tokens_to_remain = vec!["c".to_string(), "d".to_string(), "e".to_string()];
    let token_ids: Vec<String> = vec![token_to_burn.clone()]
        .iter()
        .cloned()
        .chain(tokens_to_remain.iter().cloned())
        .collect();

    contract
        .mint_copies(&ali, Some(token_ids), meta::banana(), 4)
        .unwrap();

    contract.burn_one(&ali, token_to_burn.clone()).unwrap();

    // Check balances
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(&ali), U256::from(3));

    // Check burned tokens
    assert!(contract.owner_of(&token_to_burn).is_none());
    assert!(contract.token_meta(&token_to_burn).is_none());

    // Check rest of tokens.
    for token_id in &tokens_to_remain {
        assert_eq!(&contract.owner_of(token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(token_id).unwrap(), &meta::banana());
    }
}
#[test]
fn test_transfer_token() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();
    let token_id: TokenId = "apple".to_string();

    contract
        .mint_one(&ali, Some(token_id.clone()), meta::apple())
        .unwrap();

    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(&ali), U256::from(1));
    assert_eq!(contract.balance_of(&bob), U256::from(0));

    contract.transfer_token(&ali, &bob, &token_id).unwrap();

    assert_eq!(contract.balance_of(&ali), U256::from(0));
    assert_eq!(contract.balance_of(&bob), U256::from(1));
    assert_eq!(contract.owner_of(&token_id).unwrap(), bob);
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali: Key = ali_pk.to_account_hash().into();
    let bob: Key = bob_pk.to_account_hash().into();
    let tokens_to_transfer = vec!["a".to_string(), "b".to_string()];
    let tokens_to_remain = vec!["c".to_string(), "d".to_string(), "e".to_string()];
    let token_ids: Vec<String> = tokens_to_transfer
        .iter()
        .cloned()
        .chain(tokens_to_remain.iter().cloned())
        .collect();

    contract
        .mint_copies(&ali, Some(token_ids), meta::banana(), 5)
        .unwrap();

    contract
        .transfer_many_tokens(&ali, &bob, &tokens_to_transfer)
        .unwrap();

    assert_eq!(contract.balance_of(&ali), U256::from(3));
    assert_eq!(contract.balance_of(&bob), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(5));

    // Check ali tokens.
    for token_id in &tokens_to_remain {
        assert_eq!(&contract.owner_of(token_id).unwrap(), &ali);
    }

    // Check bob tokens.
    for token_id in &tokens_to_transfer {
        assert_eq!(&contract.owner_of(token_id).unwrap(), &bob);
    }
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
