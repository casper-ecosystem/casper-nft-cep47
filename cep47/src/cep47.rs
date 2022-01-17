use crate::{
    data::{self, Allowances, Metadata, OwnedTokens, Owners},
    event::CEP47Event,
    Meta, TokenId,
};
use alloc::{string::String, vec::Vec};
use casper_types::{ApiError, Key, U256};
use contract_utils::{ContractContext, ContractStorage};
use core::convert::TryInto;

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
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
        OwnedTokens::instance().get_balances(&owner)
    }

    fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        Owners::instance().get(&token_id)
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        Metadata::instance().get(&token_id)
    }

    fn set_token_meta(&mut self, token_id: TokenId, meta: Meta) -> Result<(), Error> {
        if self.owner_of(token_id).is_none() {
            return Err(Error::TokenIdDoesntExist);
        };

        let metadata_dict = Metadata::instance();
        metadata_dict.set(&token_id, meta);

        self.emit(CEP47Event::MetadataUpdate { token_id });
        Ok(())
    }

    fn get_token_by_index(&self, owner: Key, index: U256) -> Option<TokenId> {
        OwnedTokens::instance().get_token_by_index(&owner, &index)
    }

    fn validate_token_ids(&self, token_ids: Vec<TokenId>) -> bool {
        for token_id in &token_ids {
            if self.owner_of(*token_id).is_some() {
                return false;
            }
        }
        true
    }

    fn mint(
        &mut self,
        recipient: Key,
        token_ids: Vec<TokenId>,
        token_metas: Vec<Meta>,
    ) -> Result<Vec<TokenId>, Error> {
        if token_ids.len() != token_metas.len() {
            return Err(Error::WrongArguments);
        };

        for token_id in &token_ids {
            if self.owner_of(*token_id).is_some() {
                return Err(Error::TokenIdAlreadyExists);
            }
        }

        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let metadata_dict = Metadata::instance();

        for (token_id, token_meta) in token_ids.iter().zip(&token_metas) {
            metadata_dict.set(token_id, token_meta.clone());
            owners_dict.set(token_id, recipient);
            owned_tokens_dict.set_token(&recipient, token_id);
        }

        let minted_tokens_count: U256 = From::<u64>::from(token_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_add(minted_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(CEP47Event::Mint {
            recipient,
            token_ids: token_ids.clone(),
        });
        Ok(token_ids)
    }

    fn mint_copies(
        &mut self,
        recipient: Key,
        token_ids: Vec<TokenId>,
        token_meta: Meta,
        count: u32,
    ) -> Result<Vec<TokenId>, Error> {
        let token_metas = vec![token_meta; count.try_into().unwrap()];
        self.mint(recipient, token_ids, token_metas)
    }

    fn burn(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let spender = self.get_caller();
        if spender != owner {
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
            }
        }
        self.burn_internal(owner, token_ids)
    }

    fn burn_internal(&mut self, owner: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let metadata_dict = Metadata::instance();
        let allowances_dict = Allowances::instance();

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
        }

        for token_id in &token_ids {
            owned_tokens_dict.remove_token(&owner, token_id);
            metadata_dict.remove(token_id);
            owners_dict.remove(token_id);
            allowances_dict.remove(&owner, token_id);
        }

        let burnt_tokens_count: U256 = From::<u64>::from(token_ids.len().try_into().unwrap());
        let new_total_supply = data::total_supply()
            .checked_sub(burnt_tokens_count)
            .unwrap();
        data::set_total_supply(new_total_supply);

        self.emit(CEP47Event::Burn { owner, token_ids });
        Ok(())
    }

    fn approve(&mut self, spender: Key, token_ids: Vec<TokenId>) -> Result<(), Error> {
        let caller = self.get_caller();
        for token_id in &token_ids {
            match self.owner_of(*token_id) {
                None => return Err(Error::WrongArguments),
                Some(owner) if owner != caller => return Err(Error::PermissionDenied),
                Some(_) => Allowances::instance().set(&caller, token_id, spender),
            }
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
        let spender = self.get_caller();

        if owner != spender {
            let allowances_dict = Allowances::instance();
            for token_id in &token_ids {
                if !self.is_approved(owner, *token_id, spender) {
                    return Err(Error::PermissionDenied);
                }
                allowances_dict.remove(&owner, token_id);
            }
        }
        self.transfer_from_internal(owner, recipient, token_ids)
    }

    fn transfer_from_internal(
        &mut self,
        owner: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();

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
        }

        for token_id in &token_ids {
            owned_tokens_dict.remove_token(&owner, token_id);
            owned_tokens_dict.set_token(&recipient, token_id);
            owners_dict.set(token_id, recipient);
        }

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
        false
    }

    fn emit(&mut self, event: CEP47Event) {
        data::emit(&event);
    }
}
