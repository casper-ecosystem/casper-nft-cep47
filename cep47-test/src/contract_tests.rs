use crate::cep47::{CEP47TestContract, Meta, Sender, TestConfig, TokenId};
use casper_types::U256;
use maplit::btreemap;

pub struct CasperCEP47Contract {
    config: TestConfig,
}

impl CEP47TestContract for CasperCEP47Contract {
    fn config(&self) -> &TestConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut TestConfig {
        &mut self.config
    }
}

impl CasperCEP47Contract {
    fn new() -> Self {
        let config = Self::deploy(
            "CasperNFT",
            "CNFT",
            btreemap! {"origin".to_string() => "fire".to_string()},
            "dragons-nft.wasm",
        );
        Self { config }
    }
}

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
    let contract = CasperCEP47Contract::new();

    assert_eq!(contract.name(), "CasperNFT");
    assert_eq!(contract.symbol(), "CNFT");
    assert_eq!(
        contract.meta(),
        btreemap! {"origin".to_string() => "fire".to_string()}
    );
    assert_eq!(contract.total_supply(), U256::zero());
}

#[test]
fn test_token_meta() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_meta = meta::red_dragon();
    contract.mint_one(
        ali.clone(),
        token_meta.clone(),
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    let ali_token_meta = contract.token_meta(ali_tokens[0].clone());

    assert_eq!(ali_token_meta, Some(token_meta));
}

#[test]
fn test_mint_one() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_meta = meta::red_dragon();
    contract.mint_one(
        ali.clone(),
        token_meta,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(ali.clone()), U256::one());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::one());
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_meta = meta::gold_dragon();
    contract.mint_copies(
        ali.clone(),
        token_meta,
        U256::from(3),
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(3));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali.clone()));
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali.clone()));
}

#[test]
fn test_burn_many() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::blue_dragon(),
        meta::black_dragon(),
        meta::red_dragon(),
    ];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let mut ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(4));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(4));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(4));

    contract.burn_many(
        ali.clone(),
        vec![
            ali_tokens.first().unwrap().clone(),
            ali_tokens.last().unwrap().clone(),
        ],
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));

    ali_tokens = contract.tokens(ali.clone());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));
}

#[test]
fn test_burn_one() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::red_dragon()];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );

    let mut ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(2));

    contract.burn_one(
        ali.clone(),
        ali_tokens.first().unwrap().clone(),
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );
    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(ali.clone()), U256::from(1));

    ali_tokens = contract.tokens(ali.clone());
    assert_eq!(U256::from(ali_tokens.len() as u64), U256::from(1));
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let bob = contract.config().accounts.get(2).unwrap().clone();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    contract.transfer_token(
        ali.clone(),
        bob.clone(),
        ali_tokens[1].clone(),
        Sender(ali.clone().to_account_hash()),
    );

    assert_eq!(contract.balance_of(ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(1));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob.clone()));
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let bob = contract.config().accounts.get(2).unwrap().clone();
    let token_metas: Vec<Meta> = vec![
        meta::gold_dragon(),
        meta::black_dragon(),
        meta::black_dragon(),
    ];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    contract.transfer_many_tokens(
        ali.clone(),
        bob.clone(),
        ali_tokens[..2].to_vec(),
        Sender(ali.clone().to_account_hash()),
    );

    assert_eq!(contract.balance_of(ali.clone()), U256::from(1));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[2]), Some(ali.clone()));
}

#[test]
fn test_transfer_all_tokens() {
    let mut contract = CasperCEP47Contract::new();
    let ali = contract.config().accounts.get(1).unwrap().clone();
    let bob = contract.config().accounts.get(2).unwrap().clone();
    let token_metas: Vec<Meta> = vec![meta::gold_dragon(), meta::blue_dragon()];
    contract.mint_many(
        ali.clone(),
        token_metas,
        Sender(
            contract
                .config()
                .accounts
                .first()
                .unwrap()
                .clone()
                .to_account_hash(),
        ),
    );
    let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
    assert_eq!(contract.balance_of(ali.clone()), U256::from(2));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(0));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(ali.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(ali.clone()));

    contract.transfer_all_tokens(
        ali.clone(),
        bob.clone(),
        Sender(ali.clone().to_account_hash()),
    );
    assert_eq!(contract.balance_of(ali.clone()), U256::from(0));
    assert_eq!(contract.balance_of(bob.clone()), U256::from(2));
    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.owner_of(&ali_tokens[0]), Some(bob.clone()));
    assert_eq!(contract.owner_of(&ali_tokens[1]), Some(bob.clone()));
}
