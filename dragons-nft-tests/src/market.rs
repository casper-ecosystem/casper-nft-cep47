use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext};
use casper_types::{account::AccountHash, runtime_args, PublicKey, RuntimeArgs};

pub struct MarketTest {
    pub market_hash: Hash,
}

impl MarketTest {
    pub fn deploy(context: &mut TestContext, deployer: AccountHash) -> Self {
        let session_code = Code::from("marketplace.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(deployer)
            .with_authorization_keys(&[deployer])
            .build();
        context.run(session);
        let market_hash = context
            .query(deployer, &["marketplace_contract_package_hash".to_string()])
            .unwrap()
            .into_t()
            .unwrap();
        Self { market_hash }
    }

    // fn call(
    //     &self,
    //     context: &mut TestContext,
    //     account: &AccountHash,
    //     method: &str,
    //     args: RuntimeArgs,
    // ) {
    //     let code = Code::Hash(self.market_hash, method.to_string());
    //     let session = SessionBuilder::new(code, args)
    //         .with_address(*account)
    //         .with_authorization_keys(&[*account])
    //         .build();
    //     context.run(session);
    // }

    pub fn call_test(&self, context: &mut TestContext, account: &PublicKey, seller: &PublicKey) {
        let session_code = Code::from("send_tokens.wasm");
        let session_args = runtime_args! {
            "marketplace_contract" => self.market_hash,
            "seller" => seller.clone(),
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account.to_account_hash())
            .with_authorization_keys(&[account.to_account_hash()])
            .build();
        context.run(session);
    }
}
