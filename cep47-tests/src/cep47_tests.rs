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
    let token_id = TokenId::from("0");
    let token_meta = meta::red_dragon();

    token.mint_one(Sender(owner), user, token_meta.clone());

    let user_token_meta = token.token_meta(token_id.clone());
    assert_eq!(user_token_meta.unwrap(), token_meta);

    let first_user_token = token.get_token_by_index(Key::Account(user), U256::zero());
    assert_eq!(first_user_token, Some(token_id));
}

#[test]
fn test_mint_one() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("0");
    let token_meta = meta::red_dragon();

    token.mint_one(Sender(owner), user, token_meta);
    let first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    assert_eq!(first_user_token, Some(token_id.clone()));
    assert_eq!(token.total_supply(), U256::one());
    assert_eq!(token.balance_of(Key::Account(user)), U256::one());
    assert_eq!(second_user_token, None);
    assert_eq!(token.owner_of(token_id).unwrap(), Key::Account(user));
}

#[test]
fn test_mint_copies() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_meta = meta::red_dragon();
    token.mint_copies(Sender(owner), user, token_meta, 3);
    let first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    let third_user_token = token.get_token_by_index(Key::Account(user), U256::from(2));
    let fourth_user_token = token.get_token_by_index(Key::Account(user), U256::from(3));
    assert_eq!(token.total_supply(), U256::from(3));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(3));
    assert_eq!(
        token.owner_of(first_user_token.clone().unwrap()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(second_user_token.clone().unwrap()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(third_user_token.clone().unwrap()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(first_user_token, Some(TokenId::from("0")));
    assert_eq!(second_user_token, Some(TokenId::from("1")));
    assert_eq!(third_user_token, Some(TokenId::from("2")));
    assert_eq!(fourth_user_token, None);
}

#[test]
fn test_mint_many() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
    token.mint_many(Sender(owner), user, token_metas);
    let first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    let third_user_token = token.get_token_by_index(Key::Account(user), U256::from(2));
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));
    assert_eq!(
        token.owner_of(first_user_token.clone().unwrap()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(
        token.owner_of(second_user_token.clone().unwrap()).unwrap(),
        Key::Account(user)
    );
    assert_eq!(first_user_token, Some(TokenId::from("0")));
    assert_eq!(second_user_token, Some(TokenId::from("1")));
    assert_eq!(third_user_token, None);
}

#[test]
fn test_burn_many() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![
        meta::red_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::gold_dragon(),
    ];

    token.mint_many(Sender(owner), user, token_metas);

    token.burn_many(
        Sender(user),
        user,
        vec![TokenId::from("0"), TokenId::from("3")],
    );
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));

    let new_first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let new_second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    let new_third_user_token = token.get_token_by_index(Key::Account(user), U256::from(2));
    let new_fourth_user_token = token.get_token_by_index(Key::Account(user), U256::from(3));
    assert_eq!(new_first_user_token, Some(TokenId::from("2")));
    assert_eq!(new_second_user_token, Some(TokenId::from("1")));
    assert_eq!(new_third_user_token, None);
    assert_eq!(new_fourth_user_token, None);
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

    token.mint_many(Sender(owner), user, token_metas);

    token.approve(
        Sender(user),
        owner,
        vec![TokenId::from("0"), TokenId::from("2")],
    );
    token.burn_many(
        Sender(owner),
        user,
        vec![TokenId::from("0"), TokenId::from("2")],
    );
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(2));

    let new_first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let new_second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    let new_third_user_token = token.get_token_by_index(Key::Account(user), U256::from(2));
    let new_fourth_user_token = token.get_token_by_index(Key::Account(user), U256::from(3));
    assert_eq!(new_first_user_token, Some(TokenId::from("3")));
    assert_eq!(new_second_user_token, Some(TokenId::from("1")));
    assert_eq!(new_third_user_token, None);
    assert_eq!(new_fourth_user_token, None);
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

    token.mint_many(Sender(owner), user, token_metas);

    token.burn_many(
        Sender(owner),
        user,
        vec![TokenId::from("0"), TokenId::from("1")],
    );
}

#[test]
fn test_burn_one() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
    token.mint_many(Sender(owner), user, token_metas);

    token.burn_one(Sender(user), user, TokenId::from("0"));
    assert_eq!(token.total_supply(), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(1));

    let new_first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let new_second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    assert_eq!(new_first_user_token, Some(TokenId::from("1")));
    assert_eq!(new_second_user_token, None);
}

#[test]
fn test_unique_id_after_burn_and_mint() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];
    token.mint_many(Sender(owner), user, token_metas.clone());

    assert_eq!(token.total_supply(), U256::from(2));
    token.burn_one(Sender(user), user, TokenId::from("1"));
    assert_eq!(token.total_supply(), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(1));
    token.mint_many(Sender(owner), user, token_metas);

    let new_first_user_token = token.get_token_by_index(Key::Account(user), U256::from(0));
    let new_second_user_token = token.get_token_by_index(Key::Account(user), U256::from(1));
    let new_third_user_token = token.get_token_by_index(Key::Account(user), U256::from(2));
    let new_fourth_user_token = token.get_token_by_index(Key::Account(user), U256::from(3));
    assert_eq!(token.balance_of(Key::Account(user)), U256::from(3));
    assert_eq!(token.total_supply(), U256::from(3));
    assert_eq!(new_first_user_token, Some(TokenId::from("0")));
    assert_eq!(new_second_user_token, Some(TokenId::from("2")));
    assert_eq!(new_third_user_token, Some(TokenId::from("3")));
    assert_eq!(new_fourth_user_token, None);
}

#[test]
fn test_transfer_token() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, token_metas);

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(
        token.owner_of(TokenId::from("0")).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(TokenId::from("1")).unwrap(),
        Key::Account(ali)
    );

    token.transfer(Sender(ali), bob, vec![TokenId::from("0")]);
    let new_first_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(0));
    let new_second_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(1));
    let new_first_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(0));
    let new_second_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(1));
    println!("{:?}", new_first_ali_token);
    println!("{:?}", new_second_ali_token);
    println!("{:?}", new_first_bob_token);
    println!("{:?}", new_second_bob_token);
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(bob)), U256::from(1));
    assert_eq!(
        token.owner_of(new_first_ali_token.unwrap()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(new_first_bob_token.unwrap()).unwrap(),
        Key::Account(bob)
    );
    assert_eq!(new_second_ali_token, None);
    assert_eq!(new_second_bob_token, None);
}

#[test]
fn test_transfer_from_tokens_with_approve() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, token_metas);
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(
        token.owner_of(TokenId::from("0")).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(TokenId::from("1")).unwrap(),
        Key::Account(ali)
    );
    token.approve(Sender(ali), owner, vec![TokenId::from("1")]);
    token.transfer_from(Sender(owner), ali, bob, vec![TokenId::from("1")]);
    let new_first_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(0));
    let new_second_ali_token = token.get_token_by_index(Key::Account(ali), U256::from(1));
    let new_first_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(0));
    let new_second_bob_token = token.get_token_by_index(Key::Account(bob), U256::from(1));
    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(1));
    assert_eq!(token.balance_of(Key::Account(bob)), U256::from(1));
    assert_eq!(
        token.owner_of(new_first_ali_token.unwrap()).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(new_first_bob_token.unwrap()).unwrap(),
        Key::Account(bob)
    );
    assert_eq!(new_second_ali_token, None);
    assert_eq!(new_second_bob_token, None);
}

#[test]
#[should_panic]
fn test_transfer_from_tokens_without_approve() {
    let (env, token, owner) = deploy();
    let ali = env.next_user();
    let bob = env.next_user();
    let token_metas = vec![meta::red_dragon(), meta::gold_dragon()];

    token.mint_many(Sender(owner), ali, token_metas);

    assert_eq!(token.total_supply(), U256::from(2));
    assert_eq!(token.balance_of(Key::Account(ali)), U256::from(2));
    assert_eq!(
        token.owner_of(TokenId::from("0")).unwrap(),
        Key::Account(ali)
    );
    assert_eq!(
        token.owner_of(TokenId::from("0")).unwrap(),
        Key::Account(ali)
    );
    token.transfer_from(Sender(owner), ali, bob, vec![TokenId::from("0")]);
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

    token.mint_many(Sender(owner), user, token_metas);

    token.approve(
        Sender(user),
        owner,
        vec![TokenId::from("0"), TokenId::from("3")],
    );
    assert_eq!(
        token.get_approved(user, TokenId::from("0")).unwrap(),
        Key::Account(owner)
    );
    assert_eq!(
        token.get_approved(user, TokenId::from("3")).unwrap(),
        Key::Account(owner)
    );
}

#[test]
fn test_token_metadata_update() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::from("0");

    token.mint_one(Sender(owner), user, meta::red_dragon());

    token.update_token_meta(Sender(owner), token_id.clone(), meta::gold_dragon());
    assert_eq!(token.token_meta(token_id).unwrap(), meta::gold_dragon());
}
