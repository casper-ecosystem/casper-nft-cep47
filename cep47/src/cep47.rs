use crate::{
    data::{self, Allowances, Balances, Metadata, OwnedTokens, Owners},
    event::CEP47Event,
    Meta, TokenId,
};
use alloc::{string::String, vec::Vec};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, ApiError, Key, U256};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    ArgumentsError = 2,
    TokenIdAlreadyExists = 3,
    TokenIdDoesntExist = 4,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait CEP47<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, name: String, symbol: String, meta: Meta) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_meta(meta);
        data::set_total_supply(U256::zero());
        Owners::init();
        OwnedTokens::init();
        Metadata::init();
        Balances::init();
        Allowances::init();
    }

    fn name(&self) -> String {
        data::name()
    }

    fn symbol(&self) -> String {
        data::symbol()
    }

    fn meta(&self) -> Meta {
        data::meta()
    }

    fn total_supply(&self) -> U256 {
        data::total_supply()
    }

    fn balance_of(&self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        Owners::instance().get(&token_id)
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        Metadata::instance().get(&token_id)
    }

    fn set_token_meta(&mut self, token_id: TokenId, meta: Meta) -> Result<(), Error> {
        if self.owner_of(token_id.clone()).is_none() {
            return Err(Error::TokenIdAlreadyExists);
        };

        let metadata_dict = Metadata::instance();
        metadata_dict.set(&token_id, meta);

        self.emit(CEP47Event::MetadataUpdate { token_id });
        Ok(())
    }

    fn get_token_by_index(&self, owner: Key, index: u32) -> Option<TokenId> {
        OwnedTokens::instance().get_token_by_index(&owner, &index)
    }

    fn generate_token_ids(&mut self, n: u32) -> Vec<TokenId> {
        let block_time = runtime::get_blocktime();
        let mut token_ids = Vec::new();
        let nonce = data::nonce();
        for i in nonce..nonce + n {
            let mut bytes: Vec<u8> = block_time.to_bytes().unwrap_or_revert();
            bytes.append(&mut i.to_bytes().unwrap_or_revert());
            let hash = runtime::blake2b(bytes);
            token_ids.push(hex::encode(hash));
        }
        data::set_nonce(nonce + n);
        token_ids
    }

    fn validate_token_ids(&self, token_ids: Vec<TokenId>) -> bool {
        for token_id in &token_ids {
            if self.owner_of(token_id.clone()).is_some() {
                return false;
            }
        }
        true
    }

    fn mint(
        &mut self,
        recipient: Key,
        token_ids: Option<Vec<TokenId>>,
        token_metas: Vec<Meta>,
    ) -> Result<(), Error> {
        let unique_token_ids = match token_ids {
            // Validate token_ids and metas.
            Some(token_ids) => {
                if token_ids.len() != token_metas.len() {
                    return Err(Error::ArgumentsError);
                };
                let valid = self.validate_token_ids(token_ids.clone());
                if !valid {
                    return Err(Error::TokenIdAlreadyExists);
                };
                token_ids
            }
            None => self.generate_token_ids(token_metas.len() as u32),
        };

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let metadata_dict = Metadata::instance();
        let balances_dict = Balances::instance();

        for (token_id, token_meta) in unique_token_ids.iter().zip(&token_metas) {
            metadata_dict.set(token_id, token_meta.clone());
            owners_dict.set(token_id, recipient);
            owned_tokens_dict.set_token(&recipient, token_id.clone());
        }

        let minted_tokens_count = U256::from(unique_token_ids.len() as u64);
        let prev_balance = balances_dict.get(&recipient);
        let new_balance = prev_balance + minted_tokens_count;
        balances_dict.set(&recipient, new_balance);

        let new_total_supply = data::total_supply() + minted_tokens_count;
        data::set_total_supply(new_total_supply);

        self.emit(CEP47Event::Mint {
            recipient,
            token_ids: unique_token_ids,
        });
        Ok(())
    }

    fn mint_copies(
        &mut self,
        recipient: Key,
        token_ids: Option<Vec<TokenId>>,
        token_meta: Meta,
        count: u32,
    ) -> Result<(), Error> {
        if let Some(token_ids) = &token_ids {
            if token_ids.len() != count as usize {
                return Err(Error::ArgumentsError);
            }
        }
        let token_metas = vec![token_meta; count as usize];
        self.mint(recipient, token_ids, token_metas)
    }

    fn burn(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let metadata_dict = Metadata::instance();
        let balances_dict = Balances::instance();
        let allowances_dict = Allowances::instance();

        let spender = self.get_caller();

        for token_id in &token_ids {
            if spender != owner && !self.is_approved(owner, token_id.clone(), spender) {
                return Err(Error::PermissionDenied);
            }
            match owners_dict.get(token_id) {
                Some(owner_of_key) => {
                    if owner_of_key != owner {
                        return Err(Error::PermissionDenied);
                    }
                }
                None => {
                    return Err(Error::TokenIdDoesntExist);
                }
            }
            owned_tokens_dict.remove_token(&owner, token_id.clone());
            metadata_dict.remove(token_id);
            owners_dict.remove(token_id);
            allowances_dict.remove(&owner, token_id);
        }

        let owner_balance = balances_dict.get(&owner);
        let new_owner_balance = owner_balance - U256::from(token_ids.len() as u64);
        balances_dict.set(&owner, new_owner_balance);

        let burnt_tokens_count = U256::from(token_ids.len() as u64);
        let new_total_supply = data::total_supply() - burnt_tokens_count;
        data::set_total_supply(new_total_supply);

        self.emit(CEP47Event::Burn { owner, token_ids });
        Ok(())
    }

    fn approve(&mut self, spender: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.get_caller();
        for token_id in &token_ids {
            let owner = self.owner_of(token_id.clone());
            if owner.is_none() {
                return Err(Error::ArgumentsError);
            }
            if owner.unwrap() != caller {
                return Err(Error::PermissionDenied);
            }
            Allowances::instance().set(&caller, token_id, spender);
        }
        self.emit(CEP47Event::Approve {
            owner: caller,
            spender,
            token_ids,
        });
        Ok(())
    }

    fn get_approved(&self, owner: Key, token_id: TokenId) -> Option<Key> {
        Allowances::instance().get(&owner, &token_id)
    }

    fn transfer(&mut self, recipient: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        self.transfer_from(self.get_caller(), recipient, token_ids)
    }

    fn transfer_from(
        &mut self,
        owner: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let allowances_dict = Allowances::instance();
        let spender = self.get_caller();
        if owner != spender {
            for token_id in &token_ids {
                if !self.is_approved(owner, token_id.clone(), spender) {
                    return Err(Error::PermissionDenied);
                }
                allowances_dict.remove(&owner, token_id);
            }
        }

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let balances_dict = Balances::instance();

        for token_id in &token_ids {
            match owners_dict.get(token_id) {
                Some(owner_of_key) => {
                    if owner_of_key != owner {
                        return Err(Error::PermissionDenied);
                    }
                }
                None => {
                    return Err(Error::TokenIdDoesntExist);
                }
            }
            owned_tokens_dict.remove_token(&owner, token_id.clone());
            owned_tokens_dict.set_token(&recipient, token_id.clone());
            owners_dict.set(token_id, recipient);
        }
        let owner_balance = balances_dict.get(&owner);
        let new_owner_balance = owner_balance - U256::from(token_ids.len() as u64);
        balances_dict.set(&owner, new_owner_balance);

        let recipient_balance = balances_dict.get(&recipient);
        let new_recipient_balance = recipient_balance + U256::from(token_ids.len() as u64);
        balances_dict.set(&recipient, new_recipient_balance);

        self.emit(CEP47Event::Transfer {
            sender: owner,
            recipient,
            token_ids,
        });
        Ok(())
    }

    fn is_approved(&self, owner: Key, token_id: TokenId, spender: Key) -> bool {
        let allowances_dict = Allowances::instance();
        if let Some(spender_of) = allowances_dict.get(&owner, &token_id) {
            if spender_of == spender {
                return true;
            }
        }
        return false;
    }

    fn emit(&mut self, event: CEP47Event) {
        data::emit(&event);
    }
}
