use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AsymmetricType, CLTyped, PublicKey,
    RuntimeArgs, U256, U512,
};

pub mod token_cfg {
    pub const NAME: &str = "CasperNFT";
    pub const SYMBOL: &str = "CNFT";
    pub const URI: &str = "https://casper.network/network";
}

pub const CASPERCEP47_CONTRACT: &str = "caspercep47_contract";
pub const CASPERCEP47_CONTRACT_HASH: &str = "caspercep47_contract_hash";

pub struct CasperCEP47Contract {
    pub context: TestContext,
    pub caspercep47_hash: Hash,
    pub account: AccountHash,
}

pub type TokenId = String;
pub type URI = String;

impl CasperCEP47Contract {
    pub fn deploy() -> Self {
        let account = PublicKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let mut context = TestContextBuilder::new()
            .with_public_key(account, U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("cep47.wasm");
        let session_args = runtime_args! {
            "token_name" => token_cfg::NAME,
            "token_symbol" => token_cfg::SYMBOL,
            "token_uri" => token_cfg::URI
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account.to_account_hash())
            .with_authorization_keys(&[account.to_account_hash()])
            .build();
        context.run(session);
        let caspercep47_hash = context
            .query(
                account.to_account_hash(),
                &[CASPERCEP47_CONTRACT_HASH.to_string()],
            )
            .unwrap()
            .into_t()
            .unwrap();

        Self {
            context,
            caspercep47_hash,
            account: account.to_account_hash(),
        }
    }

    fn call(&mut self, method: &str, args: RuntimeArgs) {
        let code = Code::Hash(self.caspercep47_hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(self.account)
            .with_authorization_keys(&[self.account])
            .build();
        self.context.run(session);
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self.context.query(
            self.account,
            &[CASPERCEP47_CONTRACT.to_string(), name.to_string()],
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

    pub fn uri(&self) -> URI {
        self.query_contract("uri").unwrap()
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

    pub fn token_uri(&self, token_id: TokenId) -> Option<URI> {
        self.query_contract(uri_key(&token_id).as_str())
    }

    pub fn mint_one(&mut self, recipient: PublicKey, token_uri: URI) {
        self.call(
            "mint_one",
            runtime_args! {
                "recipient" => recipient,
                "token_uri" => token_uri
            },
        );
    }

    pub fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256) {
        self.call(
            "mint_copies",
            runtime_args! {
                "recipient" => recipient,
                "token_uri" => token_uri,
                "count" => count
            },
        );
    }

    pub fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>) {
        self.call(
            "mint_many",
            runtime_args! {
                "recipient" => recipient,
                "token_uris" => token_uris
            },
        );
    }

    pub fn transfer_token(&mut self, sender: PublicKey, recipient: PublicKey, token_id: TokenId) {
        self.call(
            "transfer_token",
            runtime_args! {
                "sender" => sender,
                "recipient" => recipient,
                "token_id" => token_id
            },
        );
    }

    pub fn transfer_many_tokens(
        &mut self,
        sender: PublicKey,
        recipient: PublicKey,
        token_ids: Vec<TokenId>,
    ) {
        self.call(
            "transfer_many_tokens",
            runtime_args! {
                "sender" => sender,
                "recipient" => recipient,
                "token_ids" => token_ids
            },
        );
    }

    pub fn transfer_all_tokens(&mut self, sender: PublicKey, recipient: PublicKey) {
        self.call(
            "transfer_all_tokens",
            runtime_args! {
                "sender" => sender,
                "recipient" => recipient
            },
        );
    }

    pub fn attach(&mut self, token_uref: URef, recipient: PublicKey) {
        self.call(
            "attach",
            runtime_args! {
                "token_uref" => token_uref,
                "recipient" => recipient
            },
        );
    }

    pub fn detach(&mut self, owner: PublicKey, recipient: String) {
        self.call(
            "detach",
            runtime_args! {
                "owner" => owner,
                "token_id" => token_id
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

fn uri_key(token_id: &TokenId) -> String {
    format!("uris_{}", token_id)
}

fn token_key(account: &AccountHash) -> String {
    format!("tokens_{}", account)
}
