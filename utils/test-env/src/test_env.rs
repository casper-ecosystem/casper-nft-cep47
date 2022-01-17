use std::sync::{Arc, Mutex};

use casper_engine_test_support::{InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, CLTyped, Key, PublicKey, RuntimeArgs, SecretKey,
};

use crate::utils::{deploy, fund_account, query, query_dictionary_item, DeploySource};

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

    pub fn run(&self, sender: AccountHash, session_code: DeploySource, session_args: RuntimeArgs) {
        deploy(
            &mut self.state.lock().unwrap().builder,
            &sender,
            &session_code,
            session_args,
            true,
            None,
        )
    }

    pub fn next_user(&self) -> AccountHash {
        self.state.lock().unwrap().next_user()
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        contract_hash: [u8; 32],
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        self.state
            .lock()
            .unwrap()
            .query_dictionary(contract_hash, dict_name.to_string(), key)
    }

    pub fn query_account_named_key<T: CLTyped + FromBytes>(
        &self,
        account: AccountHash,
        path: &[String],
    ) -> T {
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
    builder: InMemoryWasmTestBuilder,
    accounts: Vec<AccountHash>,
}

impl TestEnvState {
    pub fn new() -> TestEnvState {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
        let mut accounts = Vec::new();
        for i in 0..10u8 {
            let secret_key: SecretKey = SecretKey::ed25519_from_bytes([i; 32]).unwrap();
            let public_key: PublicKey = (&secret_key).into();
            let account_hash = AccountHash::from(&public_key);
            accounts.push(account_hash);
            builder
                .exec(fund_account(&account_hash))
                .expect_success()
                .commit();
        }

        TestEnvState { builder, accounts }
    }

    pub fn _new_with_users(user_secrets: &[[u8; 32]]) -> TestEnvState {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let mut accounts = Vec::new();
        for user_secret in user_secrets {
            let secret_key: SecretKey = SecretKey::ed25519_from_bytes(user_secret).unwrap();
            let public_key: PublicKey = (&secret_key).into();
            let account_hash = AccountHash::from(&public_key);
            accounts.push(account_hash);
            builder
                .exec(fund_account(&account_hash))
                .expect_success()
                .commit();
        }

        TestEnvState { builder, accounts }
    }

    pub fn next_user(&mut self) -> AccountHash {
        self.accounts.pop().unwrap()
    }

    pub fn _run(
        &mut self,
        sender: AccountHash,
        session_code: DeploySource,
        session_args: RuntimeArgs,
    ) {
        deploy(
            &mut self.builder,
            &sender,
            &session_code,
            session_args,
            true,
            None,
        )
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        contract_hash: [u8; 32],
        dict_name: String,
        dictionary_item_key: String,
    ) -> Option<T> {
        match query_dictionary_item(
            &self.builder,
            Key::Hash(contract_hash),
            Some(dict_name),
            dictionary_item_key,
        ) {
            Ok(value) => value
                .as_cl_value()
                .expect("should be cl value.")
                .clone()
                .into_t()
                .expect("Wrong type in query result."),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub fn query_account_named_key<T: CLTyped + FromBytes>(
        &self,
        account: AccountHash,
        path: &[String],
    ) -> T {
        query(&self.builder, Key::Account(account), path)
    }
}
