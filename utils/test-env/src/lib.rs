mod test_contract;
mod test_env;

use casper_engine_test_support::AccountHash;
pub use test_contract::TestContract;
pub use test_env::TestEnv;
pub struct Sender(pub AccountHash);
