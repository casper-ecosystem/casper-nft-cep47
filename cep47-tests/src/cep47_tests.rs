use std::collections::BTreeMap;

use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256};
use test_env::{Sender, TestEnv};

use crate::cep47_instance::{CEP47Instance, Meta, TokenId};

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";

mod meta {
    use super::{BTreeMap, Meta};
    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

    pub fn black_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "black".to_string());
        meta
    }

    pub fn gold_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "gold".to_string());
        meta
    }
}

fn deploy() -> (TestEnv, CEP47Instance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = CEP47Instance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        meta::contract_meta(),
    );
    (env, token, owner)
}

#[test]
fn test_deploy() {
    let (_, token, _) = deploy();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.meta(), meta::contract_meta());
    assert_eq!(token.total_supply(), U256::zero());
}

#[test]
fn test_token_meta() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("custom_token_id");
    let token_meta = meta::red_dragon();

    token.mint_one(
        Sender(owner),
        user,
        Some(token_id.clone()),
        token_meta.clone(),
    );

    let user_token_meta = token.token_meta(token_id.clone());
    assert_eq!(user_token_meta.unwrap(), token_meta);

    let user_tokens = token.tokens(Key::Account(user));
    assert_eq!(user_tokens, vec![token_id]);
}

#[test]
fn test_mint_one_with_random_token_id() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    token.mint_one(Sender(owner), user, None, meta::red_dragon());

    assert_eq!(token.total_supply(), U256::one());
    assert_eq!(token.balance_of(user), U256::one());

    let user_tokens: Vec<TokenId> = token.tokens(Key::Account(user));
    assert_eq!(U256::from(user_tokens.len() as u64), U256::one());
    assert_eq!(
        token.owner_of(user_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
}

#[test]
fn test_mint_one_with_set_token_id() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();

    token.mint_one(
        Sender(owner),
        user,
        Some(token_id.clone()),
        token_meta.clone(),
    );
    let user_tokens = token.tokens(Key::Account(user));
    assert_eq!(user_tokens, vec![token_id.clone()]);
    assert_eq!(token.total_supply(), U256::one());
    assert_eq!(token.balance_of(Key::Account(user)), U256::one());
    assert_eq!(U256::from(user_tokens.len() as u64), U256::one());
    assert_eq!(token.owner_of(token_id).unwrap(), Key::Account(user));
}

#[test]
#[should_panic]
fn test_mint_one_with_not_unique_token_id() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();

    token.mint_one(
        Sender(owner),
        user,
        Some(token_id.clone()),
        token_meta.clone(),
    );

    token.mint_one(
        Sender(owner),
        user,
        Some(token_id.clone()),
        token_meta.clone(),
    );
}

#[test]
fn test_mint_copies() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_meta = meta::red_dragon();

    token.mint_copies(Sender(owner), user, None, token_meta.clone(), 3);

    let user_tokens = token.tokens(Key::Account(user));
    assert_eq!(token.total_supply(), U256::from(3));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(3));
    assert_eq!(U256::from(user_tokens.len() as u64), U256::from(3));
    assert_eq!(
        token.owner_of(user_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(user_tokens.get(1).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(user_tokens.get(2).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
}

#[test]
fn test_mint_many() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), user, None, token_metas.clone());
    let user_tokens = token.tokens(Key::Account(user));
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));
    assert_eq!(U256::from(user_tokens.len() as u64), U256::from(2));
    assert_eq!(
        token.owner_of(user_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(user_tokens.get(1).unwrap().clone()).unwrap(),
        Key::Account(user)
    );
}

#[test]
fn test_burn_many() {
    // TODO: the sender should be owner of nft item or allowed party
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![
        meta::red_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::gold_dragon(),
    ];

    token.mint_many(Sender(owner), user, None, token_metas.clone());

    let mut user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    println!("{:?}", user_tokens.first().unwrap().clone());

    token.burn_many(
        Sender(user),
        user,
        vec![
            user_tokens.first().unwrap().clone(),
            user_tokens.last().unwrap().clone(),
        ],
    );
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));

    user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    assert_eq!(U256::from(user_tokens.len() as u64), U256::from(2));
}

#[test]
fn test_burn_many_from_allowance_with_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![
        meta::red_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::gold_dragon(),
    ];

    token.mint_many(Sender(owner), user, None, token_metas.clone());

    let mut user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    println!("{:?}", user_tokens.first().unwrap().clone());
    token.approve(Sender(user), owner, user_tokens.clone());
    token.burn_many(
        Sender(owner),
        user,
        vec![
            user_tokens.first().unwrap().clone(),
            user_tokens.last().unwrap().clone(),
        ],
    );
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));

    user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    assert_eq!(U256::from(user_tokens.len() as u64), U256::from(2));
}

#[test]
#[should_panic]
fn test_burn_many_from_allowance_without_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![
        meta::red_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::gold_dragon(),
    ];

    token.mint_many(Sender(owner), user, None, token_metas.clone());

    let user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    println!("{:?}", user_tokens.first().unwrap().clone());
    token.burn_many(
        Sender(owner),
        user,
        vec![
            user_tokens.first().unwrap().clone(),
            user_tokens.last().unwrap().clone(),
        ],
    );
}

#[test]
fn test_burn_one() {
    // TODO: the sender should be owner of nft item or allowed party
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
    token.mint_many(Sender(owner), user, None, token_metas.clone());

    let mut user_tokens = token.tokens(Key::Account(user));
    token.burn_one(Sender(user), user, user_tokens.first().unwrap().clone());
    assert_eq!(token.total_supply(), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(1));

    user_tokens = token.tokens(Key::Account(user));
    assert_eq!(U256::from(user_tokens.len() as u64), U256::from(1));
}

#[test]
fn test_transfer_token() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, None, token_metas.clone());
    let mut ali_tokens = token.tokens(Key::Account(ali));

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(
        token.owner_of(ali_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(ali_tokens.get(1).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );

    token.transfer(Sender(ali), bob, vec![ali_tokens.get(0).unwrap().clone()]);
    ali_tokens = token.tokens(Key::Account(ali));
    let bob_tokens = token.tokens(Key::Account(bob));
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(bob)), U256::from(1));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(1));
    assert_eq!(U256::from(bob_tokens.len() as u64), U256::from(1));
    assert_eq!(
        token.owner_of(ali_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(bob_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(bob)
    );
}

#[test]
fn test_transfer_from_tokens_with_approve() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, None, token_metas.clone());
    let mut ali_tokens = token.tokens(Key::Account(ali));

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(
        token.owner_of(ali_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(ali_tokens.get(1).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    token.approve(Sender(ali), owner, vec![ali_tokens.get(0).unwrap().clone()]);
    token.transfer_from(
        Sender(owner),
        ali,
        bob,
        vec![ali_tokens.get(0).unwrap().clone()],
    );
    ali_tokens = token.tokens(Key::Account(ali));
    let bob_tokens = token.tokens(Key::Account(bob));
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(bob)), U256::from(1));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(1));
    assert_eq!(U256::from(bob_tokens.len() as u64), U256::from(1));
    assert_eq!(
        token.owner_of(ali_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(bob_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(bob)
    );
}

#[test]
#[should_panic]
fn test_transfer_from_tokens_without_approve() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, None, token_metas.clone());
    let ali_tokens = token.tokens(Key::Account(ali));

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(
        token.owner_of(ali_tokens.get(0).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(ali_tokens.get(1).unwrap().clone()).unwrap(),
        Key::Account(ali)
    );
    token.transfer_from(
        Sender(owner),
        ali,
        bob,
        vec![ali_tokens.get(0).unwrap().clone()],
    );
}

#[test]
fn test_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![
        meta::red_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::gold_dragon(),
    ];

    token.mint_many(Sender(owner), user, None, token_metas.clone());

    let user_tokens = token.tokens(Key::Account(user));
    println!("{:?}", user_tokens);
    println!("{:?}", user_tokens.first().unwrap().clone());
    token.approve(
        Sender(user),
        owner,
        vec![
            user_tokens.get(0).unwrap().clone(),
            user_tokens.get(2).unwrap().clone(),
        ],
    );
    assert_eq!(
        token
            .get_approved(user, user_tokens.get(0).unwrap().clone())
            .unwrap(),
        Key::Account(owner)
    );
    assert_eq!(
        token
            .get_approved(user, user_tokens.get(2).unwrap().clone())
            .unwrap(),
        Key::Account(owner)
    );
}

#[test]
fn test_token_metadata_update() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("123456");

    token.mint_one(
        Sender(owner),
        user,
        Some(token_id.clone()),
        meta::red_dragon(),
    );

    token.update_token_meta(Sender(owner), token_id.clone(), meta::gold_dragon());
    assert_eq!(token.token_meta(token_id).unwrap(), meta::gold_dragon());
}
