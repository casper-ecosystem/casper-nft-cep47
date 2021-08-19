use std::collections::BTreeMap;

use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash,
    HashAddr, Key, PublicKey, RuntimeArgs, SecretKey, U256, U512,
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

const BALANCES_DICT: &str = "balances";
const TOKEN_OWNERS_DICT: &str = "owners";
const METADATA_DICT: &str = "metadata";

pub struct CasperCEP47Contract {
    pub context: TestContext,
    pub hash: Hash,
    pub admin: AccountHash,
    pub ali: AccountHash,
    pub bob: AccountHash,
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
            admin: admin.to_account_hash(),
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
        }
    }

    pub fn deploy_secondary_contract(&mut self, wasm: &str, args: RuntimeArgs) -> (Hash, HashAddr) {
        let session_code = Code::from(wasm);
        let session = SessionBuilder::new(session_code, args)
            .with_address(self.admin)
            .with_authorization_keys(&[self.admin])
            .build();
        self.context.run(session);
        let hash = self
            .context
            .query(self.admin, &["owning_contract_hash".to_string()])
            .unwrap()
            .into_t()
            .unwrap();
        let package = self
            .context
            .query(self.admin, &["owning_contract_pack".to_string()])
            .unwrap()
            .into_t()
            .unwrap();
        (hash, package)
    }

    fn call(&mut self, sender: &AccountHash, method: &str, args: RuntimeArgs) {
        let account = *sender;
        let code = Code::Hash(self.hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(account)
            .with_authorization_keys(&[account])
            .build();
        self.context.run(session);
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.admin, &[CONTRACT_KEY.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn query_dictionary_value<T: CLTyped + FromBytes>(
        &self,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        match self.context.query_dictionary_item(
            Key::Hash(self.hash),
            Some(dict_name.to_string()),
            key,
        ) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value: Option<T> = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("is not expected type."));
                value
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
        self.query_contract("total_supply").unwrap()
    }

    pub fn owner_of(&self, token_id: &TokenId) -> Option<Key> {
        self.query_dictionary_value::<Key>(TOKEN_OWNERS_DICT, token_id.clone())
    }

    pub fn balance_of(&self, owner: &Key) -> U256 {
        let value: Option<U256> =
            self.query_dictionary_value(BALANCES_DICT, Self::key_to_str(owner));
        value.unwrap_or_default()
    }

    pub fn token_meta(&self, token_id: &TokenId) -> Option<Meta> {
        self.query_dictionary_value(METADATA_DICT, token_id.clone())
    }

    pub fn mint_one(
        &mut self,
        recipient: &Key,
        token_id: Option<&TokenId>,
        token_meta: &Meta,
        sender: &AccountHash,
    ) {
        self.call(
            sender,
            "mint_one",
            runtime_args! {
                "recipient" => *recipient,
                "token_id" => token_id.cloned(),
                "token_meta" => token_meta.clone()
            },
        );
    }

    pub fn mint_copies(
        &mut self,
        recipient: &Key,
        token_ids: Option<&Vec<TokenId>>,
        token_meta: &Meta,
        count: u32,
        sender: &AccountHash,
    ) {
        self.call(
            sender,
            "mint_copies",
            runtime_args! {
                "recipient" => *recipient,
                "token_ids" => token_ids.cloned(),
                "token_meta" => token_meta.clone(),
                "count" => count
            },
        );
    }

    pub fn mint_many(
        &mut self,
        recipient: &Key,
        token_ids: Option<&Vec<TokenId>>,
        token_metas: &Vec<Meta>,
        sender: &AccountHash,
    ) {
        self.call(
            sender,
            "mint_many",
            runtime_args! {
                "recipient" => *recipient,
                "token_ids" => token_ids.cloned(),
                "token_metas" => token_metas.clone(),
            },
        );
    }

    pub fn burn_many(&mut self, owner: &Key, token_ids: &Vec<TokenId>, sender: &AccountHash) {
        self.call(
            sender,
            "burn_many",
            runtime_args! {
                "owner" => *owner,
                "token_ids" => token_ids.clone()
            },
        );
    }

    pub fn burn_one(&mut self, owner: &Key, token_id: &TokenId, sender: &AccountHash) {
        self.call(
            sender,
            "burn_one",
            runtime_args! {
                "owner" => *owner,
                "token_id" => token_id.clone()
            },
        );
    }

    pub fn transfer_token(&mut self, recipient: &Key, token_id: &TokenId, sender: &AccountHash) {
        self.call(
            sender,
            "transfer_token",
            runtime_args! {
                "recipient" => *recipient,
                "token_id" => token_id.clone()
            },
        );
    }

    pub fn transfer_token_from_contract(
        &mut self,
        contract_manager: &AccountHash,
        contract_hash: &Hash,
        recipient: &Key,
        token_id: &TokenId,
    ) {
        let code = Code::Hash(*contract_hash, "transfer_token".to_string());
        let nft_package: ContractPackageHash =
            self.query_contract("contract_package_hash").unwrap();
        let session = SessionBuilder::new(
            code,
            runtime_args! {
                "nft" => nft_package,
                "sender" => Key::Hash(*contract_hash),
                "recipient" => *recipient,
                "token_id" => token_id.clone()
            },
        )
        .with_address(*contract_manager)
        .with_authorization_keys(&[*contract_manager])
        .build();
        self.context.run(session);
    }

    pub fn transfer_many_tokens(
        &mut self,
        recipient: &Key,
        token_ids: &Vec<TokenId>,
        sender: &AccountHash,
    ) {
        self.call(
            sender,
            "transfer_many_tokens",
            runtime_args! {
                "recipient" => *recipient,
                "token_ids" => token_ids.clone()
            },
        );
    }

    pub fn update_token_metadata(&mut self, token_id: &TokenId, meta: &Meta, sender: &AccountHash) {
        self.call(
            sender,
            "update_token_metadata",
            runtime_args! {
                "token_id" => token_id.clone(),
                "token_meta" => meta.clone()
            },
        );
    }

    pub fn pause(&mut self, sender: &AccountHash) {
        self.call(sender, "pause", runtime_args! {});
    }

    pub fn unpause(&mut self, sender: &AccountHash) {
        self.call(sender, "unpause", runtime_args! {});
    }

    pub fn is_paused(&mut self) -> bool {
        self.query_contract("paused").unwrap()
    }

    fn key_to_str(key: &Key) -> String {
        match key {
            Key::Account(account) => account.to_string(),
            Key::Hash(package) => hex::encode(package),
            _ => panic!(),
        }
    }
}
