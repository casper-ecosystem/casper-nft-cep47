#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::BTreeMap;

use casper_types::{ApiError, AsymmetricType, PublicKey, URef, U256};

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[cfg(test)]
pub mod tests;

pub type TokenId = String;
pub type Meta = BTreeMap<String, String>;

#[repr(u16)]
pub enum Error {
    PermissionDenied = 2, // 65538
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait WithStorage<Storage: CEP47Storage> {
    fn storage(&self) -> &Storage;
    fn storage_mut(&mut self) -> &mut Storage;
}

pub trait CEP47Contract<Storage: CEP47Storage>: WithStorage<Storage> {
    // Metadata
    fn name(&self) -> String {
        self.storage().name()
    }

    fn symbol(&self) -> String {
        self.storage().symbol()
    }

    fn meta(&self) -> Meta {
        self.storage().meta()
    }

    // Getters
    fn balance_of(&self, owner: PublicKey) -> U256 {
        self.storage().balance_of(owner)
    }

    fn owner_of(&self, token_id: TokenId) -> Option<PublicKey> {
        self.storage().onwer_of(token_id)
    }

    fn total_supply(&self) -> U256 {
        self.storage().total_supply()
    }

    fn token_meta(&self, token_id: TokenId) -> Option<Meta> {
        self.storage().token_meta(token_id)
    }

    fn tokens(&self, owner: PublicKey) -> Vec<TokenId> {
        self.storage().get_tokens(owner)
    }

    fn is_paused(&self) -> bool {
        self.storage().is_paused()
    }

    fn pause(&mut self) {
        self.storage_mut().pause();
    }

    fn unpause(&mut self) {
        self.storage_mut().unpause();
    }

    // Minter function.
    // Guarded by the entrypoint group.
    fn mint_one(&mut self, recipient: PublicKey, token_meta: Meta) {
        self.storage_mut()
            .mint_copies(recipient, token_meta, U256::one());
    }

    fn mint_many(&mut self, recipient: PublicKey, token_metas: Vec<Meta>) {
        self.storage_mut().mint_many(recipient, token_metas);
    }

    fn mint_copies(&mut self, recipient: PublicKey, token_meta: Meta, count: U256) {
        self.storage_mut().mint_copies(recipient, token_meta, count);
    }

    fn burn_one(&mut self, owner: PublicKey, token_id: TokenId) {
        self.storage_mut().burn_one(owner, token_id);
    }

    fn burn_many(&mut self, owner: PublicKey, token_ids: Vec<TokenId>) {
        self.storage_mut().burn_many(owner, token_ids);
    }

    // Transfer functions.
    fn transfer_token(
        &mut self,
        sender: PublicKey,
        recipient: PublicKey,
        token_id: TokenId,
    ) -> Result<(), Error> {
        let paused = self.storage().is_paused();
        if paused {
            return Err(Error::PermissionDenied);
        }
        // 1. Load tokens owned by the sender.
        let mut sender_tokens = self.storage().get_tokens(sender.clone());
        // 2. Assert that token_id is in sender_tokens.
        if !sender_tokens.contains(&token_id) {
            return Err(Error::PermissionDenied);
        }
        // 3. Remove token_id from sender_tokens.
        sender_tokens.retain(|x| x.clone() != token_id);
        self.storage_mut().set_tokens(sender, sender_tokens);

        // 4. Add token_id to the recipient tokens
        let mut recipient_tokens = self.storage().get_tokens(recipient.clone());
        recipient_tokens.push(token_id);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
        Ok(())
    }

    fn transfer_many_tokens(
        &mut self,
        sender: PublicKey,
        recipient: PublicKey,
        token_ids: Vec<TokenId>,
    ) -> Result<(), Error> {
        let paused = self.storage().is_paused();
        if paused {
            return Err(Error::PermissionDenied);
        }
        let mut sender_tokens = self.storage().get_tokens(sender.clone());
        println!("Sender tokens: {:?}", sender_tokens);

        for token_id in token_ids.iter() {
            if !sender_tokens.contains(token_id) {
                return Err(Error::PermissionDenied);
            }
            sender_tokens.retain(|x| x.clone() != token_id.clone());
        }
        let mut recipient_tokens = self.storage().get_tokens(recipient.clone());
        recipient_tokens.append(&mut token_ids.clone());
        self.storage_mut().set_tokens(sender, sender_tokens);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
        Ok(())
    }

    fn transfer_all_tokens(
        &mut self,
        sender: PublicKey,
        recipient: PublicKey,
    ) -> Result<(), Error> {
        let paused = self.storage().is_paused();
        if paused {
            return Err(Error::PermissionDenied);
        }
        let mut sender_tokens = self.storage().get_tokens(sender.clone());
        let mut recipient_tokens = self.storage().get_tokens(recipient.clone());
        recipient_tokens.append(&mut sender_tokens);

        self.storage_mut().set_tokens(sender, sender_tokens);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
        Ok(())
    }
}

pub trait CEP47Storage {
    // Metadata.
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn meta(&self) -> Meta;

    // Getters
    fn balance_of(&self, owner: PublicKey) -> U256;
    fn onwer_of(&self, token_id: TokenId) -> Option<PublicKey>;
    fn total_supply(&self) -> U256;
    fn token_meta(&self, token_id: TokenId) -> Option<Meta>;

    // Controls
    fn is_paused(&self) -> bool;
    fn pause(&mut self);
    fn unpause(&mut self);

    // Setters
    fn get_tokens(&self, owner: PublicKey) -> Vec<TokenId>;
    fn set_tokens(&mut self, owner: PublicKey, token_ids: Vec<TokenId>);
    fn mint_many(&mut self, recipient: PublicKey, token_metas: Vec<Meta>);
    fn mint_copies(&mut self, recipient: PublicKey, token_metas: Meta, count: U256);
    fn burn_one(&mut self, owner: PublicKey, token_id: TokenId);
    fn burn_many(&mut self, owner: PublicKey, token_ids: Vec<TokenId>);
}
