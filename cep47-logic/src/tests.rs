use casper_types::AccessRights;
use rand::Rng;

use crate::{
    AsymmetricType, CEP47Contract, CEP47Storage, Meta, PublicKey, TokenId, URef, WithStorage, U256, Key
};
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

    fn balance_of(&self, owner: Key) -> U256 {
        let owner_balance = self.balances.get(&owner);
        if owner_balance.is_none() {
            U256::from(0)
        } else {
            owner_balance.unwrap().clone()
        }
    }

    fn onwer_of(&self, token_id: TokenId) -> Option<Key> {
        let owner = self.belongs_to.get(&token_id);
        if owner.is_some() {
            Some(owner.unwrap().clone())
        } else {
            None
        }
    }

    fn total_supply(&self) -> U256 {
        self.total_supply
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        let meta = self.token_metas.get(&token_id);
        if meta.is_some() {
            Some(meta.unwrap().clone())
        } else {
            None
        }
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

    fn get_tokens(&self, owner: Key) -> Vec<TokenId> {
        let owner_tokens = self.tokens.get(&owner);
        if owner_tokens.is_none() {
            Vec::<TokenId>::new()
        } else {
            owner_tokens.unwrap().clone()
        }
    }

    fn set_tokens(&mut self, owner: Key, token_ids: Vec<TokenId>) {
        let owner_new_balance = U256::from(token_ids.len() as u64);

        let owner_tokens = self.get_tokens(owner.clone());
        for token_id in owner_tokens.clone() {
            self.belongs_to.remove(&token_id);
        }
        for token_id in token_ids.clone() {
            self.belongs_to.insert(token_id, owner.clone());
        }

        self.tokens.insert(owner.clone(), token_ids.clone());
        self.balances.insert(owner, owner_new_balance);
    }

    fn mint_many(&mut self, recipient: Key, token_metas: Vec<Meta>) {
        let recipient_balance = self.balances.get(&recipient);
        let recipient_tokens = self.tokens.get(&recipient);
        let mut recipient_new_balance = if recipient_balance.is_none() {
            U256::from(0)
        } else {
            recipient_balance.unwrap().clone()
        };
        let mut recipient_new_tokens = if recipient_tokens.is_none() {
            Vec::<TokenId>::new()
        } else {
            recipient_tokens.unwrap().clone()
        };

        let mut hasher = DefaultHasher::new();

        for token_meta in token_metas.clone() {
            let token_info = (self.meta.clone(), token_meta.clone(), self.total_supply());
            Hash::hash(&token_info, &mut hasher);
            let token_id: TokenId = TokenId::from(hasher.finish().to_string());
            self.token_metas.insert(token_id.clone(), token_meta);
            recipient_new_tokens.push(token_id.clone());
            self.belongs_to.insert(token_id, recipient.clone());
            recipient_new_balance = recipient_new_balance + 1;
            self.total_supply = self.total_supply + 1;
        }
        self.balances
            .insert(recipient.clone(), recipient_new_balance);
        self.tokens.insert(recipient, recipient_new_tokens);
    }

    fn mint_copies(&mut self, recipient: Key, token_meta: Meta, count: U256) {
        let token_metas: Vec<Meta> = vec![token_meta; count.as_usize()];
        self.mint_many(recipient, token_metas);
    }

    fn burn_many(&mut self, owner: Key, token_ids: Vec<TokenId>) {
        let owner_tokens = self.tokens.get(&owner);
        let owner_balance = self.balances.get(&owner);
        let mut owner_new_balance = if owner_balance.is_none() {
            U256::from(0)
        } else {
            owner_balance.unwrap().clone()
        };
        let mut owner_new_tokens = if owner_tokens.is_none() {
            Vec::<TokenId>::new()
        } else {
            owner_tokens.unwrap().clone()
        };

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
        self.balances.insert(owner.clone(), owner_new_balance);
        self.tokens.insert(owner, owner_new_tokens);
    }

    fn burn_one(&mut self, owner: Key, token_id: TokenId) {
        let owner_tokens = self.tokens.get(&owner);
        let owner_balance = self.balances.get(&owner);
        let owner_new_balance = if owner_balance.is_none() {
            U256::from(0)
        } else {
            owner_balance.unwrap().clone()
        };
        let mut owner_new_tokens = if owner_tokens.is_none() {
            Vec::<TokenId>::new()
        } else {
            owner_tokens.unwrap().clone()
        };
        let index = owner_new_tokens
            .iter()
            .position(|x| *x == token_id.clone())
            .unwrap();
        owner_new_tokens.remove(index);
        self.token_metas.remove(&token_id.clone());
        self.belongs_to.remove(&token_id.clone());
        self.total_supply = self.total_supply - 1;
        self.balances.insert(owner.clone(), owner_new_balance - 1);
        self.tokens.insert(owner, owner_new_tokens);
    }

    fn new_uref(&mut self, token_id: TokenId) -> Option<URef> {
        let mut rng = rand::thread_rng();
        let val: [u8; 32] = rng.gen();
        let uref = URef::new(val, AccessRights::READ_ADD_WRITE);
        if self.urefs.contains_key(&uref) {
            None
        } else {
            self.urefs.insert(uref, token_id);
            Some(uref)
        }
    }

    fn del_uref(&mut self, token_uref: URef) -> Option<TokenId> {
        let token_id = self.token_id(token_uref);
        if token_id.is_none() {
            None
        } else {
            let token_id = token_id.unwrap();
            self.urefs.remove(&token_uref);
            Some(token_id)
        }
    }

    fn token_id(&self, token_uref: URef) -> Option<TokenId> {
        self.urefs.get(&token_uref).map(|x| x.clone())
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
    let ali = Key::Account(ali_pk.to_account_hash());
    let bob = Key::Account(bob_pk.to_account_hash());
    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_many(ali.clone(), vec![meta::apple()]);
    contract.mint_many(bob.clone(), vec![meta::banana(), meta::orange()]);
    assert_eq!(contract.total_supply(), U256::from(3));

    let ali_balance = contract.balance_of(ali.clone());
    assert_eq!(ali_balance, U256::from(1));
    let bob_balance = contract.balance_of(bob.clone());
    assert_eq!(bob_balance, U256::from(2));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali);
    let ali_first_token_meta: Meta = contract
        .token_meta(ali_tokens.get(0).unwrap().clone())
        .unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());

    let bob_tokens: Vec<TokenId> = contract.tokens(bob);
    let bob_first_token_meta: Meta = contract
        .token_meta(bob_tokens.get(1).unwrap().clone())
        .unwrap();
    assert_eq!(bob_first_token_meta, meta::orange());
}
#[test]
fn test_mint_copies() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());
    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_copies(ali.clone(), meta::apple(), U256::from(7));
    assert_eq!(contract.total_supply(), U256::from(7));

    let ali_balance = contract.balance_of(ali.clone());
    assert_eq!(ali_balance, U256::from(7));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali);
    let ali_first_token_meta: Meta = contract
        .token_meta(ali_tokens.get(0).unwrap().clone())
        .unwrap();
    let ali_third_token_meta: Meta = contract
        .token_meta(ali_tokens.get(2).unwrap().clone())
        .unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());
    assert_eq!(ali_third_token_meta, meta::apple());
}
#[test]
fn test_burn_many() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());

    assert_eq!(contract.total_supply(), U256::from(0));

    contract.mint_many(
        ali.clone(),
        vec![meta::banana(), meta::orange(), meta::apple()],
    );
    assert_eq!(contract.total_supply(), U256::from(3));

    let ali_balance = contract.balance_of(ali.clone());
    assert_eq!(ali_balance, U256::from(3));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let banana = ali_tokens.get(0).unwrap().clone();
    let orange = ali_tokens.get(1).unwrap().clone();
    let apple = ali_tokens.get(2).unwrap().clone();

    contract.burn_many(ali.clone(), vec![banana.clone(), apple.clone()]);
    let ali_tokens_after_burn = contract.tokens(ali.clone());
    assert_eq!(ali_tokens_after_burn, vec![orange.clone()]);

    assert!(contract.token_meta(banana.clone()).is_none());
    assert!(contract.token_meta(orange.clone()).is_some());
    assert!(contract.token_meta(apple.clone()).is_none());

    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(ali), U256::from(1));
}
#[test]
fn test_burn_one() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());

    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_many(ali.clone(), vec![meta::banana(), meta::orange()]);
    assert_eq!(contract.total_supply(), U256::from(2));

    let mut ali_balance = contract.balance_of(ali.clone());
    assert_eq!(ali_balance, U256::from(2));

    let mut ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    contract.burn_one(ali.clone(), ali_tokens.get(0).unwrap().clone());
    let mut ali_first_token_meta = contract.token_meta(ali_tokens.get(0).unwrap().clone());
    assert_eq!(ali_first_token_meta, None);

    ali_tokens = contract.tokens(ali.clone());
    ali_first_token_meta = contract.token_meta(ali_tokens.get(0).unwrap().clone());
    assert_eq!(ali_first_token_meta, Some(meta::orange()));

    assert_eq!(contract.total_supply(), U256::from(1));
    ali_balance = contract.balance_of(ali);
    assert_eq!(ali_balance, U256::from(1));
}
#[test]
fn test_transfer_token() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());
    let bob = Key::Account(bob_pk.to_account_hash());

    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_one(ali.clone(), meta::apple());
    assert_eq!(contract.total_supply(), U256::from(1));

    let mut ali_balance = contract.balance_of(ali.clone());
    let mut bob_balance = contract.balance_of(bob.clone());
    assert_eq!(ali_balance, U256::from(1));
    assert_eq!(bob_balance, U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let ali_first_token_id: TokenId = ali_tokens.get(0).unwrap().clone();
    let ali_first_token_meta: Meta = contract.token_meta(ali_first_token_id.clone()).unwrap();
    assert_eq!(ali_first_token_meta, meta::apple());

    let transfer_res =
        contract.transfer_token(ali.clone(), bob.clone(), ali_first_token_id.clone());
    assert!(transfer_res.is_ok());
    ali_balance = contract.balance_of(ali);
    bob_balance = contract.balance_of(bob.clone());
    assert_eq!(ali_balance, U256::from(0));
    assert_eq!(bob_balance, U256::from(1));

    let owner_of_first_token_id = contract.owner_of(ali_first_token_id);
    assert_eq!(owner_of_first_token_id.unwrap(), bob);
}
#[test]
fn test_transfer_all_tokens() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());
    let bob = Key::Account(bob_pk.to_account_hash());

    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_many(ali.clone(), vec![meta::apple(), meta::banana()]);
    contract.mint_one(ali.clone(), meta::apple());
    assert_eq!(contract.total_supply(), U256::from(3));

    let mut ali_balance = contract.balance_of(ali.clone());
    let mut bob_balance = contract.balance_of(bob.clone());
    assert_eq!(ali_balance, U256::from(3));
    assert_eq!(bob_balance, U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
    let ali_second_token_meta: Meta = contract.token_meta(ali_second_token_id.clone()).unwrap();
    assert_eq!(ali_second_token_meta, meta::banana());

    let transfer_res = contract.transfer_all_tokens(ali.clone(), bob.clone());
    assert!(transfer_res.is_ok());

    ali_balance = contract.balance_of(ali);
    bob_balance = contract.balance_of(bob.clone());
    assert_eq!(ali_balance, U256::from(0));
    assert_eq!(bob_balance, U256::from(3));

    let owner_of_second_token_id = contract.owner_of(ali_second_token_id);
    assert_eq!(owner_of_second_token_id.unwrap(), bob);
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());
    let bob = Key::Account(bob_pk.to_account_hash());

    assert_eq!(contract.total_supply(), U256::from(0));
    contract.mint_many(ali.clone(), vec![meta::apple(), meta::banana()]);
    contract.mint_copies(ali.clone(), meta::apple(), U256::from(3));
    assert_eq!(contract.total_supply(), U256::from(5));

    assert_eq!(contract.balance_of(ali.clone()), U256::from(5));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(0));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
    let ali_second_token_meta: Meta = contract.token_meta(ali_second_token_id.clone()).unwrap();
    let ali_third_token_id: TokenId = ali_tokens.get(2).unwrap().clone();
    let ali_third_token_meta: Meta = contract.token_meta(ali_third_token_id.clone()).unwrap();
    assert_eq!(ali_second_token_meta, meta::banana());
    assert_eq!(ali_third_token_meta, meta::apple());

    let transfer_res = contract.transfer_many_tokens(
        ali.clone(),
        bob.clone(),
        vec![ali_second_token_id.clone(), ali_third_token_id.clone()],
    );
    assert!(transfer_res.is_ok());
    assert_eq!(contract.balance_of(ali), U256::from(3));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(2));

    let owner_of_second_token_id = contract.owner_of(ali_second_token_id);
    let owner_of_third_token_id = contract.owner_of(ali_third_token_id);
    assert_eq!(owner_of_second_token_id.unwrap(), bob);
    assert_eq!(owner_of_third_token_id.unwrap(), bob);
}

#[test]
fn test_attach_and_detach() {
    let mut contract = TestContract::new();
    let ali_pk = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let bob_pk = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
    let ali = Key::Account(ali_pk.to_account_hash());
    let bob = Key::Account(bob_pk.to_account_hash());

    contract.mint_one(ali.clone(), meta::banana());
    let token_id: TokenId = contract.tokens(ali.clone())[0].clone();

    let token_uref: URef = contract.detach(ali.clone(), token_id.clone()).unwrap();
    assert_eq!(contract.balance_of(ali.clone()), U256::zero());
    assert_eq!(contract.total_supply(), U256::one());
    assert!(contract.tokens(ali).is_empty());

    assert_eq!(contract.token_id(token_uref.clone()), token_id.clone());
    assert_eq!(
        contract.token_meta(token_id.clone()).unwrap(),
        meta::banana()
    );

    contract.attach(token_uref, bob.clone());
    assert_eq!(contract.balance_of(bob.clone()), U256::one());
    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.tokens(bob), vec![token_id]);
}
