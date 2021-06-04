use crate::cep47::{token_cfg, CasperCEP47Contract, TokenId, URI};
use crate::market::MarketTest;
use casper_types::{AsymmetricType, PublicKey, SecretKey, U256};

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
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uri = URI::from("MonaLisa");
    contract.mint_one(ali.clone(), token_uri.clone());

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let ali_token_uri = contract.token_uri(ali_tokens[0].clone());

    assert_eq!(ali_token_uri, Some(token_uri));
}

#[test]
fn test_mint_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uri = URI::from("MonaLisa");
    contract.mint_one(ali.clone(), token_uri);

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(ali.clone()), U256::one());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uri = URI::from("Casper Golden Card");
    contract.mint_copies(ali.clone(), token_uri, U256::from(3));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(3));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali));
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uris: Vec<URI> = vec![URI::from("Casper Golden Card"), URI::from("Mona Lisa")];
    contract.mint_many(ali.clone(), token_uris);

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali));
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519_from_bytes([3u8; 32]).unwrap().into();
    let bob: PublicKey = SecretKey::ed25519_from_bytes([5u8; 32]).unwrap().into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(ali.clone(), token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    contract.transfer_token(ali.clone(), bob.clone(), ali_tokens[1].clone());

    assert_eq!(contract.balance_of(ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(1));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519_from_bytes([3u8; 32]).unwrap().into();
    let bob: PublicKey = SecretKey::ed25519_from_bytes([5u8; 32]).unwrap().into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
        URI::from("Casper Bronze Card"),
    ];
    contract.mint_many(ali.clone(), token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    contract.transfer_many_tokens(ali.clone(), bob.clone(), ali_tokens[..2].to_vec());

    assert_eq!(contract.balance_of(ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali));
}

#[test]
fn test_transfer_all_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519_from_bytes([3u8; 32]).unwrap().into();
    let bob: PublicKey = SecretKey::ed25519_from_bytes([5u8; 32]).unwrap().into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(ali.clone(), token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(0));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali.clone()));

    contract.transfer_all_tokens(ali.clone(), bob.clone());
    assert_eq!(contract.balance_of(ali.clone()), U256::from(0));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
}

#[test]
fn test_marketplace() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    let market = MarketTest::deploy(&mut contract.context, contract.admin.to_account_hash());
    market.call_test(&mut contract.context, &contract.ali, &contract.admin);
    contract.mint_many(contract.ali.clone(), token_uris);
}
