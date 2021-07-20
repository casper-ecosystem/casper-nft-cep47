use std::collections::BTreeMap;

use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, PublicKey, RuntimeArgs,
    SecretKey, URef, U256, U512,
};

pub type TokenId = String;
pub type Meta = BTreeMap<String, String>;

pub const CONTRACT_KEY_SUFFIX: &str = "_contract";
pub const CONTRACT_HASH_KEY_SUFFIX: &str = "_contract_hash";
pub const CONTRACT_NAME_KEY: &str = "name";
pub const CONTRACT_SYMBOL_KEY: &str = "symbol";
pub const CONTRACT_META_KEY: &str = "meta";
pub const CONTRACT_TOTAL_SUPPLY_KEY: &str = "total_supply";

pub struct Sender(pub AccountHash);

pub struct TestConfig {
    pub context: TestContext,
    pub contract_hash: Hash,
    pub contract_key: String,
    pub accounts: Vec<PublicKey>,
}

pub trait CEP47TestContract {
    fn config(&self) -> &TestConfig;
    fn config_mut(&mut self) -> &mut TestConfig;

    fn deploy(token_name: &str, token_symbol: &str, token_meta: Meta, wasm: &str) -> TestConfig {
        let admin: PublicKey = (&SecretKey::ed25519_from_bytes([1u8; 32]).unwrap()).into();
        let ali: PublicKey = (&SecretKey::ed25519_from_bytes([3u8; 32]).unwrap()).into();
        let bob: PublicKey = (&SecretKey::ed25519_from_bytes([5u8; 32]).unwrap()).into();
        let mut context = TestContextBuilder::new()
            .with_public_key(admin.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from(wasm);
        let session_args = runtime_args! {
            "token_name" => token_name,
            "token_symbol" => token_symbol,
            "token_meta" => token_meta
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(admin.to_account_hash())
            .with_authorization_keys(&[admin.to_account_hash()])
            .build();
        context.run(session);
        let contract_hash = context
            .query(
                admin.to_account_hash(),
                &[format!("{}{}", token_name, CONTRACT_HASH_KEY_SUFFIX)],
            )
            .unwrap()
            .into_t()
            .unwrap();
        let contract_key = format!("{}{}", token_name, CONTRACT_KEY_SUFFIX);

        TestConfig {
            context,
            contract_hash,
            contract_key,
            accounts: vec![admin, ali, bob],
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.config().contract_hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.config_mut().context.run(session);
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self.config().context.query(
            self.config().accounts.first().unwrap().to_account_hash(),
            &[self.config().contract_key.clone(), name.to_string()],
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

    fn name(&self) -> String {
        self.query_contract(CONTRACT_NAME_KEY).unwrap()
    }

    fn symbol(&self) -> String {
        self.query_contract(CONTRACT_SYMBOL_KEY).unwrap()
    }

    fn meta(&self) -> Meta {
        self.query_contract(CONTRACT_META_KEY).unwrap()
    }

    fn total_supply(&self) -> U256 {
        self.query_contract(CONTRACT_TOTAL_SUPPLY_KEY)
            .unwrap_or_default()
    }

    fn owner_of(&self, token_id: &TokenId) -> Option<PublicKey> {
        self.query_contract(owner_key(&token_id).as_str())
    }

    fn balance_of(&self, owner: PublicKey) -> U256 {
        self.query_contract(balance_key(&owner.to_account_hash()).as_str())
            .unwrap_or_default()
    }

    fn tokens(&self, owner: PublicKey) -> Vec<TokenId> {
        self.query_contract::<Vec<TokenId>>(token_key(&owner.to_account_hash()).as_str())
            .unwrap_or_default()
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        self.query_contract(meta_key(&token_id).as_str())
    }

    fn token_uref(&self, token_id: &TokenId) -> Option<URef> {
        self.query_contract(test_uref_key(&token_id).as_str())
    }

    fn mint_one(&mut self, recipient: PublicKey, token_meta: Meta, sender: Sender) {
        self.call(
            sender,
            "mint_one",
            runtime_args! {
                "recipient" => recipient,
                "token_meta" => token_meta
            },
        );
    }

    fn mint_copies(&mut self, recipient: PublicKey, token_meta: Meta, count: U256, sender: Sender) {
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

    fn mint_many(&mut self, recipient: PublicKey, token_metas: Vec<Meta>, sender: Sender) {
        self.call(
            sender,
            "mint_many",
            runtime_args! {
                "recipient" => recipient,
                "token_metas" => token_metas
            },
        );
    }

    fn burn_many(&mut self, owner: PublicKey, token_ids: Vec<TokenId>, sender: Sender) {
        self.call(
            sender,
            "burn_many",
            runtime_args! {
                "owner" => owner,
                "token_ids" => token_ids
            },
        );
    }

    fn burn_one(&mut self, owner: PublicKey, token_id: TokenId, sender: Sender) {
        self.call(
            sender,
            "burn_one",
            runtime_args! {
                "owner" => owner,
                "token_id" => token_id
            },
        );
    }

    fn transfer_token(
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

    fn transfer_many_tokens(
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

    fn transfer_all_tokens(&mut self, owner: PublicKey, recipient: PublicKey, sender: Sender) {
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
