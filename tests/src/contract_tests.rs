use crate::cep47::{token_cfg, CasperCEP47Contract, Sender, TokenId, URI};
use casper_types::U256;

#[test]
fn test_deploy() {
    let contract = CasperCEP47Contract::deploy();

    assert_eq!(contract.name(), token_cfg::NAME);
    assert_eq!(contract.symbol(), token_cfg::SYMBOL);
    assert_eq!(contract.uri(), token_cfg::URI);
    assert_eq!(contract.total_supply(), U256::zero());
}

#[test]
fn test_token_uri() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_uri = URI::from("MonaLisa");
    contract.mint_one(
        contract.ali.clone(),
        token_uri.clone(),
        Sender(contract.admin.clone().to_account_hash()),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(contract.ali.clone());
    let ali_token_uri = contract.token_uri(ali_tokens[0].clone());

    assert_eq!(ali_token_uri, Some(token_uri));
}

#[test]
fn test_mint_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_uri = URI::from("MonaLisa");
    contract.mint_one(
        contract.ali.clone(),
        token_uri,
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
    let token_uri = URI::from("Casper Golden Card");
    contract.mint_copies(
        contract.ali.clone(),
        token_uri,
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
    let token_uris: Vec<URI> = vec![URI::from("Casper Golden Card"), URI::from("Mona Lisa")];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Siver Card"),
        URI::from("Casper Bronze Card"),
        URI::from("Mona Lisa"),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
    let token_uris: Vec<URI> = vec![URI::from("Casper Golden Card"), URI::from("Mona Lisa")];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
        URI::from("Casper Bronze Card"),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(
        contract.ali.clone(),
        token_uris,
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
