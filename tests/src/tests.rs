use crate::cep47::{token_cfg, CasperCEP47Contract, TokenId, URI};
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
fn test_mint_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uri = URI::from("MonaLisa");
    contract.mint_one(ali, token_uri);

    let ali_tokens: Vec<TokenId> = contract.tokens(ali);

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(ali), U256::one());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uri = URI::from("Casper Golden Card");
    contract.mint_copies(ali, token_uri, U256::from(3));

    let ali_tokens: Vec<TokenId> = contract.tokens(ali);

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(ali), U256::from(3));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali));
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
    let token_uris: Vec<URI> = vec![URI::from("Casper Golden Card"), URI::from("Mona Lisa")];
    contract.mint_many(ali, token_uris);

    let ali_tokens: Vec<TokenId> = contract.tokens(ali);

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(ali), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali));
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519([3u8; 32]).into();
    let bob: PublicKey = SecretKey::ed25519([5u8; 32]).into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(ali, token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali);
    contract.transfer_token(ali, bob, ali_tokens[1].clone());

    assert_eq!(contract.balance_of(ali), U256::from(1));
    assert_eq!(contract.balance_of(bob), U256::from(1));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519([3u8; 32]).into();
    let bob: PublicKey = SecretKey::ed25519([5u8; 32]).into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
        URI::from("Casper Bronze Card"),
    ];
    contract.mint_many(ali, token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali);
    contract.transfer_many_tokens(ali, bob, ali_tokens[..2].to_vec());

    assert_eq!(contract.balance_of(ali), U256::from(1));
    assert_eq!(contract.balance_of(bob), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali));
}

#[test]
fn test_transfer_all_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let ali: PublicKey = SecretKey::ed25519([3u8; 32]).into();
    let bob: PublicKey = SecretKey::ed25519([5u8; 32]).into();
    let token_uris: Vec<URI> = vec![
        URI::from("Casper Golden Card"),
        URI::from("Casper Silver Card"),
    ];
    contract.mint_many(ali, token_uris);
    let ali_tokens: Vec<TokenId> = contract.tokens(ali);
    assert_eq!(contract.balance_of(ali), U256::from(2));
    assert_eq!(contract.balance_of(bob), U256::from(0));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali));

    contract.transfer_all_tokens(ali, bob);
    assert_eq!(contract.balance_of(ali), U256::from(0));
    assert_eq!(contract.balance_of(bob), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob));
}
