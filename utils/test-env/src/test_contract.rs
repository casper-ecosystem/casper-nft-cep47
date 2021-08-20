use casper_engine_test_support::{AccountHash, Code, Hash, Value};
use casper_types::{bytesrepr::FromBytes, CLTyped, RuntimeArgs};

use crate::{Sender, TestEnv};

pub struct TestContract {
    env: TestEnv,
    name: String,
    contract_owner: AccountHash,
}

impl TestContract {
    pub fn new(
        env: &TestEnv,
        wasm: &str,
        name: &str,
        sender: Sender,
        mut args: RuntimeArgs,
    ) -> TestContract {
        let Sender(contract_owner) = sender;
        let session_code = Code::from(wasm);
        args.insert("contract_name", name).unwrap();
        env.run(sender, session_code, args);

        TestContract {
            env: env.clone(),
            name: String::from(name),
            contract_owner,
        }
    }

    pub fn query_dictionary<T: CLTyped + FromBytes>(
        &self,
        dict_name: &str,
        key: String,
    ) -> Option<T> {
        self.env
            .query_dictionary(self.contract_hash(), dict_name, key)
    }

    pub fn query_named_key<T: CLTyped + FromBytes>(&self, key: String) -> T {
        let contract_name = format!("{}_contract_hash", self.name);
        self.env
            .query_account_named_key(self.contract_owner, &[contract_name, key])
            .into_t()
            .unwrap()
    }

    pub fn contract_hash(&self) -> Hash {
        let key = format!("{}_contract_hash_wrapped", self.name);
        let value: Value = self
            .env
            .query_account_named_key(self.contract_owner, &[key]);
        value.into_t().unwrap()
    }

    pub fn call_contract(&self, sender: Sender, entry_point: &str, session_args: RuntimeArgs) {
        let session_code = Code::Hash(self.contract_hash(), String::from(entry_point));
        self.env.run(sender, session_code, session_args);
    }
}
