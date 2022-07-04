use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{system::CallStackElement, Key};

use crate::ContractStorage;

pub trait ContractContext<Storage: ContractStorage> {
    fn storage(&self) -> &Storage;

    fn get_caller(&self) -> Key {
        let call_stack = self.storage().call_stack();
        let caller = call_stack.get(call_stack.len() - 2);
        element_to_key(caller.unwrap_or_revert())
    }

    fn self_addr(&self) -> Key {
        let call_stack = self.storage().call_stack();
        element_to_key(call_stack.last().unwrap_or_revert())
    }
}

fn element_to_key(element: &CallStackElement) -> Key {
    match element {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => (*contract_package_hash).into(),
    }
}
