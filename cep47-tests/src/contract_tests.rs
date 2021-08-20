use crate::cep47::{token_cfg, CasperCEP47Contract, Meta, TokenId};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};

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
    let token_id = String::from("custom_token_id");
    let token_meta = meta::red_dragon();
    let ali = Key::Account(contract.ali);

    contract.mint_one(&ali, Some(&token_id), &token_meta, &contract.admin.clone());

    assert_eq!(contract.token_meta(&token_id).unwrap(), token_meta);
    assert_eq!(contract.owner_of(&token_id).unwrap(), ali);
}

#[test]
fn test_mint_one_with_random_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::red_dragon();
    let ali = Key::Account(contract.ali);

    contract.mint_one(&ali, None, &token_meta, &contract.admin.clone());

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(&ali), U256::one());
}

#[test]
fn test_mint_one_with_set_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    let ali = Key::Account(contract.ali);

    contract.mint_one(&ali, Some(&token_id), &token_meta, &contract.admin.clone());

    assert_eq!(contract.total_supply(), U256::one());
    assert_eq!(contract.balance_of(&ali), U256::one());
    assert_eq!(contract.owner_of(&token_id).unwrap(), ali);
}

#[test]
#[should_panic]
fn test_mint_one_with_not_unique_token_id() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &Key::Account(contract.ali),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );
    contract.mint_one(
        &Key::Account(contract.ali),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );
}

#[test]
fn test_mint_copies() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_meta = meta::gold_dragon();
    let token_ids = vec![TokenId::from("a"), TokenId::from("b")];
    let ali = Key::Account(contract.ali);
    contract.mint_copies(
        &ali,
        Some(&token_ids),
        &token_meta,
        2,
        &contract.admin.clone(),
    );

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(&ali), U256::from(2));

    for token_id in token_ids {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(&token_id).unwrap(), &token_meta);
    }
}

#[test]
fn test_mint_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_metas = vec![meta::gold_dragon(), meta::black_dragon()];
    let token_ids = vec![TokenId::from("a"), TokenId::from("b")];
    let ali = Key::Account(contract.ali);
    contract.mint_many(
        &ali,
        Some(&token_ids),
        &token_metas,
        &contract.admin.clone(),
    );

    assert_eq!(contract.total_supply(), U256::from(2));
    assert_eq!(contract.balance_of(&ali), U256::from(2));

    for (token_id, token_meta) in token_ids.iter().zip(token_metas) {
        assert_eq!(&contract.owner_of(token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(token_id).unwrap(), &token_meta);
    }
}
#[test]
fn test_burn_many() {
    let mut contract = CasperCEP47Contract::deploy();
    let tokens_to_burn = vec![TokenId::from("a"), TokenId::from("b")];
    let tokens_to_keep = vec![TokenId::from("c"), TokenId::from("d"), TokenId::from("e")];
    let token_ids = tokens_to_burn
        .iter()
        .cloned()
        .chain(tokens_to_keep.iter().cloned())
        .collect();
    let token_meta = meta::black_dragon();
    let ali = Key::Account(contract.ali);

    contract.mint_copies(
        &ali,
        Some(&token_ids),
        &token_meta,
        5,
        &contract.admin.clone(),
    );

    contract.burn_many(&ali, &tokens_to_burn, &contract.admin.clone());

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(&ali), U256::from(3));

    for token_id in tokens_to_burn {
        assert!(&contract.owner_of(&token_id).is_none());
        assert!(&contract.token_meta(&token_id).is_none());
    }

    for token_id in tokens_to_keep {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(&token_id).unwrap(), &token_meta);
    }
}

#[test]
fn test_burn_one() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_to_burn = TokenId::from("a");
    let tokens_to_keep = vec![TokenId::from("c"), TokenId::from("d"), TokenId::from("e")];
    let token_ids = vec![token_to_burn.clone()]
        .into_iter()
        .chain(tokens_to_keep.iter().cloned())
        .collect();
    let token_meta = meta::black_dragon();
    let ali = Key::Account(contract.ali);

    contract.mint_copies(
        &ali,
        Some(&token_ids),
        &token_meta,
        4,
        &contract.admin.clone(),
    );

    contract.burn_one(&ali, &token_to_burn, &contract.admin.clone());

    assert_eq!(contract.total_supply(), U256::from(3));
    assert_eq!(contract.balance_of(&ali), U256::from(3));

    assert!(&contract.owner_of(&token_to_burn).is_none());
    assert!(&contract.token_meta(&token_to_burn).is_none());

    for token_id in tokens_to_keep {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
        assert_eq!(&contract.token_meta(&token_id).unwrap(), &token_meta);
    }
}

#[test]
fn test_transfer_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_to_transfer = TokenId::from("a");
    let tokens_to_keep = vec![TokenId::from("c"), TokenId::from("d"), TokenId::from("e")];
    let token_ids = vec![token_to_transfer.clone()]
        .into_iter()
        .chain(tokens_to_keep.iter().cloned())
        .collect();
    let token_meta = meta::black_dragon();
    let ali = Key::Account(contract.ali);
    let bob = Key::Account(contract.bob);

    contract.mint_copies(
        &ali,
        Some(&token_ids),
        &token_meta,
        4,
        &contract.admin.clone(),
    );

    contract.transfer_token(&bob, &token_to_transfer, &contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(4));
    assert_eq!(contract.balance_of(&ali), U256::from(3));
    assert_eq!(contract.balance_of(&bob), U256::from(1));
    assert_eq!(&contract.owner_of(&token_to_transfer).unwrap(), &bob);

    for token_id in tokens_to_keep {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
    }
}

#[test]
fn test_transfer_many_tokens() {
    let mut contract = CasperCEP47Contract::deploy();
    let tokens_to_transfer = vec![TokenId::from("a"), TokenId::from("b")];
    let tokens_to_keep = vec![TokenId::from("c"), TokenId::from("d"), TokenId::from("e")];
    let token_ids = tokens_to_transfer
        .iter()
        .cloned()
        .chain(tokens_to_keep.iter().cloned())
        .collect();
    let token_meta = meta::black_dragon();
    let ali = Key::Account(contract.ali);
    let bob = Key::Account(contract.bob);

    contract.mint_copies(
        &ali,
        Some(&token_ids),
        &token_meta,
        5,
        &contract.admin.clone(),
    );

    contract.transfer_many_tokens(&bob, &tokens_to_transfer, &contract.ali.clone());

    assert_eq!(contract.total_supply(), U256::from(5));
    assert_eq!(contract.balance_of(&ali), U256::from(3));
    assert_eq!(contract.balance_of(&bob), U256::from(2));

    for token_id in tokens_to_keep {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
    }

    for token_id in tokens_to_transfer {
        assert_eq!(&contract.owner_of(&token_id).unwrap(), &bob);
    }
}

#[test]
fn test_token_metadata_update() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    contract.mint_one(
        &Key::Account(contract.ali),
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );

    contract.update_token_metadata(&token_id, &meta::blue_dragon(), &contract.admin.clone());
    assert_eq!(contract.token_meta(&token_id).unwrap(), meta::blue_dragon());
}

#[test]
fn test_contract_owning_token() {
    let mut contract = CasperCEP47Contract::deploy();
    let (contract_hash, package) =
        contract.deploy_secondary_contract("owning-contract.wasm", runtime_args! {});
    let token_id = TokenId::from("123456");
    let token_meta = meta::red_dragon();
    let contract_address = Key::Hash(package);
    let ali = Key::Account(contract.ali);

    contract.mint_one(
        &contract_address,
        Some(&token_id),
        &token_meta,
        &contract.admin.clone(),
    );

    assert_eq!(contract.total_supply(), U256::from(1));
    assert_eq!(contract.balance_of(&contract_address), U256::from(1));
    assert_eq!(&contract.owner_of(&token_id).unwrap(), &contract_address);

    contract.transfer_token_from_contract(&contract.admin.clone(), &contract_hash, &ali, &token_id);

    assert_eq!(contract.balance_of(&contract_address), U256::from(0));
    assert_eq!(contract.balance_of(&ali), U256::from(1));
    assert_eq!(&contract.owner_of(&token_id).unwrap(), &ali);
}

#[test]
#[should_panic = "ApiError::User(1)"]
fn test_transfer_when_paused() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let ali = Key::Account(contract.ali);
    let bob = Key::Account(contract.bob);

    contract.mint_one(
        &ali,
        Some(&token_id),
        &meta::black_dragon(),
        &contract.admin.clone(),
    );
    contract.pause(&contract.admin.clone());

    // Test panics here, since contract is paused and a generic user is trying to call a non-admin function
    contract.transfer_token(&bob, &token_id, &contract.ali.clone());
}
#[test]
fn test_pausing_and_unpausing_contract() {
    let mut contract = CasperCEP47Contract::deploy();
    let token_id = TokenId::from("123456");
    let ali = Key::Account(contract.ali);
    let bob = Key::Account(contract.bob);
    let admin = contract.admin;

    contract.pause(&admin);
    contract.mint_one(&ali, Some(&token_id), &meta::black_dragon(), &admin);

    assert_eq!(contract.balance_of(&ali), U256::from(1));

    // admin functions still work even while paused
    contract.burn_one(&ali, &token_id, &admin);
    contract.mint_one(&bob, Some(&token_id), &meta::black_dragon(), &admin);

    assert_eq!(contract.balance_of(&ali), U256::from(0));
    assert_eq!(contract.balance_of(&bob), U256::from(1));

    // Transfer works after unpausing.
    contract.unpause(&admin);
    contract.transfer_token(&ali, &token_id, &contract.bob.clone());
}

#[test]
#[should_panic = "InvalidContext"]
fn test_pausing_contract_unauthorized() {
    // generic user cannot pause the contract
    let mut contract = CasperCEP47Contract::deploy();
    contract.pause(&contract.ali.clone());
}

#[test]
#[should_panic = "InvalidContext"]
fn test_unpausing_contract_unauthorized() {
    // admin can pause the contract, but generic user cannot unpause
    let mut contract = CasperCEP47Contract::deploy();
    contract.pause(&contract.admin.clone());
    contract.unpause(&contract.ali.clone());
}

#[test]
fn test_paused_field() {
    let mut contract = CasperCEP47Contract::deploy();
    assert!(!contract.is_paused());
    contract.pause(&contract.admin.clone());
    assert!(contract.is_paused());
    contract.unpause(&contract.admin.clone());
    assert!(!contract.is_paused());
}
