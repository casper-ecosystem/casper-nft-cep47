use std::sync::{Arc, Mutex};

use casper_engine_test_support::{
    AccountHash, Code, Hash, SessionBuilder, TestContext, TestContextBuilder, Value,
};
use casper_types::{bytesrepr::FromBytes, CLTyped, Key, PublicKey, RuntimeArgs, SecretKey, U512};

use crate::Sender;

#[derive(Clone)]
pub struct TestEnv {
    state: Arc<Mutex<TestEnvState>>,
}

impl TestEnv {
    pub fn new() -> TestEnv {
        TestEnv {
            state: Arc::new(Mutex::new(TestEnvState::new())),
        }
    }

    pub fn run(&self, sender: Sender, session_code: Code, session_args: RuntimeArgs) {
        self.state
            .lock()
            .unwrap()
            .run(sender, session_code, session_args);
    }

    pub fn next_user(&self) -> AccountHash {
        self.state.lock().unwrap().next_user()
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        contract_hash: Hash,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        self.state
            .lock()
            .unwrap()
            .query_dictionary(contract_hash, dict_name, key)
    }

    pub fn query_account_named_key(&self, account: AccountHash, path: &[String]) -> Value {
        self.state
            .lock()
            .unwrap()
            .query_account_named_key(account, path)
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        TestEnv::new()
    }
}

struct TestEnvState {
    context: TestContext,
    accounts: Vec<AccountHash>,
}

impl TestEnvState {
    pub fn new() -> TestEnvState {
        let mut context_builder = TestContextBuilder::new();

        let mut accounts = Vec::new();
        for i in 0..10u8 {
            let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
            let public_key: PublicKey = (&secret_key).into();
            accounts.push(AccountHash::from(&public_key));
            context_builder =
                context_builder.with_public_key(public_key, U512::from(500_000_000_000_000u64));
        }

        TestEnvState {
            context: context_builder.build(),
            accounts,
        }
    }

    pub fn next_user(&mut self) -> AccountHash {
        self.accounts.pop().unwrap()
    }

    pub fn run(&mut self, sender: Sender, session_code: Code, session_args: RuntimeArgs) {
        let Sender(sender) = sender;
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(sender)
            .with_authorization_keys(&[sender])
            .build();
        self.context.run(session);
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        contract_hash: Hash,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        match self.context.query_dictionary_item(
            Key::Hash(contract_hash),
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

    pub fn query_account_named_key(&self, account: AccountHash, path: &[String]) -> Value {
        self.context.query(account, path).unwrap()
    }
}
