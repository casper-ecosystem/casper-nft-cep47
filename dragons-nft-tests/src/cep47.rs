use std::collections::BTreeMap;

use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, PublicKey, RuntimeArgs,
    SecretKey, URef, U256, U512,
};

pub mod token_cfg {
    use super::Meta;
    use maplit::btreemap;

    pub const NAME: &str = "DragonsNFT";
    pub const SYMBOL: &str = "DRAG";

    pub fn contract_meta() -> Meta {
        btreemap! {
            "origin".to_string() => "fire".to_string()
        }
    }
}

pub const CONTRACT_KEY: &str = "DragonsNFT_contract";
pub const CONTRACT_HASH_KEY: &str = "DragonsNFT_contract_hash";

pub struct Sender(pub AccountHash);
pub struct CasperCEP47Contract {
    pub context: TestContext,
    pub hash: Hash,
    pub admin: PublicKey,
    pub ali: PublicKey,
    pub bob: PublicKey,
}

pub type TokenId = String;
pub type Meta = BTreeMap<String, String>;

impl CasperCEP47Contract {
    pub fn deploy() -> Self {
        let admin_secret = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let ali_secret = SecretKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob_secret = SecretKey::ed25519_from_bytes([5u8; 32]).unwrap();

        let admin: PublicKey = (&admin_secret).into();
        let ali: PublicKey = (&ali_secret).into();
        let bob: PublicKey = (&bob_secret).into();
        let mut context = TestContextBuilder::new()
            .with_public_key(admin.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("dragons-nft.wasm");
        let session_args = runtime_args! {
            "token_name" => token_cfg::NAME,
            "token_symbol" => token_cfg::SYMBOL,
            "token_meta" => token_cfg::contract_meta()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(admin.to_account_hash())
            .with_authorization_keys(&[admin.to_account_hash()])
            .build();
        context.run(session);
        let hash = context
            .query(admin.to_account_hash(), &[CONTRACT_HASH_KEY.to_string()])
            .unwrap()
            .into_t()
            .unwrap();

        Self {
            context,
            hash,
            admin: admin,
            ali: ali,
            bob: bob,
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self.context.query(
            self.admin.to_account_hash(),
            &[CONTRACT_KEY.to_string(), name.to_string()],
        ) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    pub fn name(&self) -> String {
        self.query_contract("name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("symbol").unwrap()
    }

    pub fn meta(&self) -> Meta {
        self.query_contract("meta").unwrap()
    }

    pub fn total_supply(&self) -> U256 {
        self.query_contract("total_supply").unwrap_or_default()
    }

    pub fn owner_of(&self, token_id: &TokenId) -> Option<PublicKey> {
        self.query_contract(owner_key(&token_id).as_str())
    }

    pub fn balance_of(&self, owner: PublicKey) -> U256 {
        self.query_contract(balance_key(&owner.to_account_hash()).as_str())
            .unwrap_or_default()
    }

    pub fn tokens(&self, owner: PublicKey) -> Vec<TokenId> {
        self.query_contract::<Vec<TokenId>>(token_key(&owner.to_account_hash()).as_str())
            .unwrap_or_default()
    }

    pub fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        self.query_contract(meta_key(&token_id).as_str())
    }

    pub fn token_uref(&self, token_id: &TokenId) -> Option<URef> {
        self.query_contract(test_uref_key(&token_id).as_str())
    }

    pub fn mint_one(&mut self, recipient: PublicKey, token_meta: Meta, sender: Sender) {
        self.call(
            sender,
            "mint_one",
            runtime_args! {
                "recipient" => recipient,
                "token_meta" => token_meta
            },
        );
    }

    pub fn mint_copies(
        &mut self,
        recipient: PublicKey,
        token_meta: Meta,
        count: U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            "mint_copies",
            runtime_args! {
                "recipient" => recipient,
                "token_meta" => token_meta,
                "count" => count
            },
        );
    }

    pub fn mint_many(&mut self, recipient: PublicKey, token_metas: Vec<Meta>, sender: Sender) {
        self.call(
            sender,
            "mint_many",
            runtime_args! {
                "recipient" => recipient,
                "token_metas" => token_metas
            },
        );
    }

    pub fn burn_many(&mut self, owner: PublicKey, token_ids: Vec<TokenId>, sender: Sender) {
        self.call(
            sender,
            "burn_many",
            runtime_args! {
                "owner" => owner,
                "token_ids" => token_ids
            },
        );
    }

    pub fn burn_one(&mut self, owner: PublicKey, token_id: TokenId, sender: Sender) {
        self.call(
            sender,
            "burn_one",
            runtime_args! {
                "owner" => owner,
                "token_id" => token_id
            },
        );
    }

    pub fn transfer_token(
        &mut self,
        owner: PublicKey,
        recipient: PublicKey,
        token_id: TokenId,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_token",
            runtime_args! {
                "sender" => owner,
                "recipient" => recipient,
                "token_id" => token_id
            },
        );
    }

    pub fn transfer_many_tokens(
        &mut self,
        owner: PublicKey,
        recipient: PublicKey,
        token_ids: Vec<TokenId>,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_many_tokens",
            runtime_args! {
                "sender" => owner,
                "recipient" => recipient,
                "token_ids" => token_ids
            },
        );
    }

    pub fn transfer_all_tokens(&mut self, owner: PublicKey, recipient: PublicKey, sender: Sender) {
        self.call(
            sender,
            "transfer_all_tokens",
            runtime_args! {
                "sender" => owner,
                "recipient" => recipient
            },
        );
    }
}

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

fn owner_key(token_id: &TokenId) -> String {
    format!("owners_{}", token_id)
}

fn meta_key(token_id: &TokenId) -> String {
    format!("metas_{}", token_id)
}

fn token_key(account: &AccountHash) -> String {
    format!("tokens_{}", account)
}

fn test_uref_key(token_id: &TokenId) -> String {
    format!("turef_{}", token_id)
}
