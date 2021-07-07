use crate::cep47::{token_cfg, CasperCEP47Contract, Meta, Sender, TokenId};
use casper_types::U256;

mod meta {
    use super::Meta;
    use maplit::btreemap;

    pub fn red_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "red".to_string()
        }
    }

    pub fn blue_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "blue".to_string()
        }
    }

    pub fn black_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "black".to_string()
        }
    }

    pub fn gold_dragon() -> Meta {
        btreemap! {
            "color".to_string() => "gold".to_string()
        }
    }
}

#[test]
fn test_deploy() {
    let contract = CasperCEP47Contract::deploy();

    assert_eq!(contract.name(), token_cfg::NAME);
    assert_eq!(contract.symbol(), token_cfg::SYMBOL);
    assert_eq!(contract.meta(), token_cfg::contract_meta());
    assert_eq!(contract.total_supply(), U256::zero());
}

#[test]
fn test_token_meta() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::red_dragon();
    contract.mint_one(
        contract.ali.clone(),
        token_meta.clone(),
        Sender(contract.admin.clone().to_account_hash()),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());
    let ali_token_meta = contract.token_meta(ali_tokens[0].clone());

    assert_eq!(ali_token_meta, Some(token_meta));
}

#[test]
fn test_mint_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::red_dragon();
    contract.mint_one(
        contract.ali.clone(),
        token_meta,
        Sender(contract.admin.clone().to_account_hash()),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::one());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(contract.ali));
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::gold_dragon();
    contract.mint_copies(
        contract.ali.clone(),
        token_meta,
        U256::from(3),
        Sender(contract.admin.clone().to_account_hash()),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(3));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(3));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.ali.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[1]),
        Some(contract.ali.clone())
    );
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(contract.ali));
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.ali.clone())
    );
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(contract.ali));
}

#[test]
fn test_burn_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::red_dragon(),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );

    let mut ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(4));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(4));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(4));

    contract.burn_many(
        contract.ali.clone(),
        vec![
            ali_tokens.first().unwrap().clone(),
            ali_tokens.last().unwrap().clone(),
        ],
        Sender(contract.admin.clone().to_account_hash()),
    );
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(2));

    ali_tokens = contract.tokens(contract.ali.clone());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
}

#[test]
fn test_burn_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );

    let mut ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));

    contract.burn_one(
        contract.ali.clone(),
        ali_tokens.first().unwrap().clone(),
        Sender(contract.admin.clone().to_account_hash()),
    );
    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(1));

    ali_tokens = contract.tokens(contract.ali.clone());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(1));
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());
    contract.transfer_token(
        contract.ali.clone(),
        contract.bob.clone(),
        ali_tokens[1].clone(),
        Sender(contract.ali.clone().to_account_hash()),
    );

    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(contract.bob.clone()), U256::from(1));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.ali.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[1]),
        Some(contract.bob.clone())
    );
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::black_dragon(),
        meta::black_dragon(),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());
    contract.transfer_many_tokens(
        contract.ali.clone(),
        contract.bob.clone(),
        ali_tokens[..2].to_vec(),
        Sender(contract.ali.clone().to_account_hash()),
    );

    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(contract.bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.bob.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[1]),
        Some(contract.bob.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[2]),
        Some(contract.ali.clone())
    );
}

#[test]
fn test_transfer_all_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        contract.ali.clone(),
        token_metas,
        Sender(contract.admin.clone().to_account_hash()),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(2));
    assert_eq!(contract.balance_of(contract.bob.clone()), U256::from(0));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.ali.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[1]),
        Some(contract.ali.clone())
    );

    contract.transfer_all_tokens(
        contract.ali.clone(),
        contract.bob.clone(),
        Sender(contract.ali.clone().to_account_hash()),
    );
    assert_eq!(contract.balance_of(contract.ali.clone()), U256::from(0));
    assert_eq!(contract.balance_of(contract.bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(
        contract.owner_of(&ali_tokens[0]),
        Some(contract.bob.clone())
    );
    assert_eq!(
        contract.owner_of(&ali_tokens[1]),
        Some(contract.bob.clone())
    );
}
