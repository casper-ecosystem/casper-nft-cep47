use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext};
use casper_types::{account::AccountHash, runtime_args, AsymmetricType, PublicKey, RuntimeArgs};

pub struct MarketTest {
    pub market_hash: Hash,
    pub account: PublicKey,
}

impl MarketTest {
    pub fn deploy(context: &mut TestContext) -> Self {
        let admin = PublicKey::ed25519_from_bytes([1u8; 32]).unwrap();
        let session_code = Code::from("marketplace.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(admin.to_account_hash())
            .with_authorization_keys(&[admin.to_account_hash()])
            .build();
        context.run(session);
        let market_hash = context
            .query(
                admin.to_account_hash(),
                &["marketplace_contract_package_hash".to_string()],
            )
            .unwrap()
            .into_t()
            .unwrap();
        Self {
            market_hash,
            account: admin,
        }
    }

    fn call(
        &self,
        context: &mut TestContext,
        account: &AccountHash,
        method: &str,
        args: RuntimeArgs,
    ) {
        let code = Code::Hash(self.market_hash, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(*account)
            .with_authorization_keys(&[*account])
            .build();
        context.run(session);
    }

    pub fn call_test(&self, context: &mut TestContext) {
        let session_code = Code::from("send_tokens.wasm");
        let session_args = runtime_args! {
            "marketplace_contract" => self.market_hash,
            "seller" => self.account.clone(),
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.account.to_account_hash())
            .with_authorization_keys(&[self.account.to_account_hash()])
            .build();
        context.run(session);
    }
}
