use casper_contract::contract_api::runtime;
use casper_types::{ApiError, Key};

use crate::{ContractContext, ContractStorage, Dict};

const ADMINS_DICT: &str = "admins";

pub trait AdminControl<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        Admins::init();
    }

    fn add_admin(&mut self, address: Key) {
        self.assert_caller_is_admin();
        self.add_admin_without_checked(address);
    }

    fn disable_admin(&mut self, address: Key) {
        self.assert_caller_is_admin();
        Admins::instance().disable_admin(&address);
    }

    fn add_admin_without_checked(&mut self, address: Key) {
        Admins::instance().add_admin(&address);
    }

    fn assert_caller_is_admin(&self) {
        let caller = self.get_caller();
        if !self.is_admin(caller) {
            runtime::revert(ApiError::User(20));
        }
    }

    fn is_admin(&self, address: Key) -> bool {
        Admins::instance().is_admin(&address)
    }
}

struct Admins {
    dict: Dict,
}

impl Admins {
    pub fn instance() -> Admins {
        Admins {
            dict: Dict::instance(ADMINS_DICT),
        }
    }
    pub fn init() {
        Dict::init(ADMINS_DICT);
    }

    pub fn is_admin(&self, key: &Key) -> bool {
        self.dict.get_by_key::<()>(key).is_some()
    }

    pub fn add_admin(&self, key: &Key) {
        self.dict.set_by_key(key, ());
    }

    pub fn disable_admin(&self, key: &Key) {
        self.dict.remove_by_key::<()>(key);
    }
}
