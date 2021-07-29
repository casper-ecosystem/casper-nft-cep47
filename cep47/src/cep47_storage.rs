use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, ApiError, Key, U256};
use cep47_logic::{CEP47Storage, Meta, TokenId};

use crate::data::{self, Balances, Metadata, OwnedTokens, Owners};

pub struct CasperCEP47Storage {}
impl CasperCEP47Storage {
    pub fn new() -> CasperCEP47Storage {
        CasperCEP47Storage {}
    }
}

impl CEP47Storage for CasperCEP47Storage {
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

    fn balance_of(&self, owner: &Key) -> U256 {
        Balances::instance().get(owner)
    }

    fn onwer_of(&self, token_id: &TokenId) -> Option<Key> {
        Owners::instance().get(token_id)
    }

    fn token_meta(&self, token_id: &TokenId) -> Option<Meta> {
        Metadata::instance().get(token_id)
    }

    fn is_paused(&self) -> bool {
        data::is_paused()
    }

    fn pause(&mut self) {
        data::pause();
    }

    fn unpause(&mut self) {
        data::unpause();
    }

    fn get_tokens(&self, owner: &Key) -> Vec<TokenId> {
        OwnedTokens::instance().get(owner)
    }

    fn set_tokens(&mut self, owner: &Key, token_ids: Vec<TokenId>) {
        // Prepare dictionaries.
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let balances_dict = Balances::instance();

        // Update the owner for each token.
        for token_id in &token_ids {
            owners_dict.set(token_id, owner.clone());
        }

        // Update balance of the owner.
        let prev_balance = balances_dict.get(&owner);
        let new_balance = U256::from(token_ids.len() as u64);
        balances_dict.set(owner, new_balance);

        // Update owner's list of tokens.
        owned_tokens_dict.set(&owner, token_ids);

        // Update total_supply.
        let new_total_supply = data::total_supply() - prev_balance + new_balance;
        data::update_total_supply(new_total_supply);
    }

    fn mint_many(&mut self, recipient: &Key, token_ids: &Vec<TokenId>, token_metas: &Vec<Meta>) {
        // Prepare dictionaries.
        let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let balances_dict = Balances::instance();
        let metadata_dict = Metadata::instance();

        // Load recipient's tokens.
        let mut recipient_tokens = owned_tokens_dict.get(&recipient);

        // Create new tokens.
        for (token_id, token_meta) in token_ids.iter().zip(token_metas) {
            // Set metadata.
            metadata_dict.set(token_id, token_meta.clone());

            // Set token owner.
            owners_dict.set(token_id, recipient.clone());

            // Update current list of recipient's tokens.
            recipient_tokens.push(token_id.clone());

            // Emit event.
            // emit_mint_one_event(&recipient, &token_id);
        }

        // Update owned tokens.
        owned_tokens_dict.set(recipient, recipient_tokens);

        // Update recipient's balance.
        let new_tokens_count: U256 = token_ids.len().into();
        let prev_balance = balances_dict.get(recipient);
        let new_balance = prev_balance + new_tokens_count;
        balances_dict.set(recipient, new_balance);

        // Update total supply.
        let new_total_supply = data::total_supply() + new_tokens_count;
        data::update_total_supply(new_total_supply);
    }

    fn burn_many(&mut self, owner: &Key, token_ids: &Vec<TokenId>) {
        // Prepare dictionaries.
        // let owners_dict = Owners::instance();
        let owned_tokens_dict = OwnedTokens::instance();
        let balances_dict = Balances::instance();
        // let metadata_dict = Metadata::instance();

        // Load owner's tokens.
        let mut owner_tokens = owned_tokens_dict.get(owner);

        // Remove tokens.
        for token_id in token_ids {
            // Remove token form the onwer's list.
            // Make sure that token is owned by the recipient.
            let index = owner_tokens
                .iter()
                .position(|x| x == token_id)
                .unwrap_or_revert();
            owner_tokens.remove(index);

            // TODO: Remove meta.

            // TODO: Remove ownership.

            // Emit event.
            // emit_burn_one_event(&owner, &token_id);
        }

        // Decrement owner's balance.
        balances_dict.set(owner, owner_tokens.len().into());

        // Update owner's tokens.
        owned_tokens_dict.set(owner, owner_tokens);

        // Decrement total supply.
        let remove_tokens_count: U256 = token_ids.len().into();
        let new_total_supply = data::total_supply() - remove_tokens_count;
        data::update_total_supply(new_total_supply);
    }

    fn update_token_metadata(&mut self, token_id: &TokenId, meta: Meta) {
        let metadata_dict = Metadata::instance();
        let current_meta = metadata_dict.get(token_id);
        match current_meta {
            None => runtime::revert(ApiError::None),
            Some(_) => metadata_dict.set(token_id, meta),
        };
    }

    fn gen_token_ids(&mut self, n: u32) -> Vec<TokenId> {
        let block_time = runtime::get_blocktime();
        let mut token_ids = Vec::new();
        let nonce = data::get_nonce();
        for i in nonce..nonce + n {
            let mut bytes: Vec<u8> = block_time.to_bytes().unwrap_or_revert();
            bytes.append(&mut i.to_bytes().unwrap_or_revert());
            let hash = runtime::blake2b(bytes);
            token_ids.push(hex::encode(hash));
        }
        data::set_nonce(nonce + n);
        token_ids
    }

    fn validate_token_ids(&self, token_ids: &Vec<TokenId>) -> bool {
        for token_id in token_ids {
            if self.onwer_of(token_id).is_some() {
                return false;
            }
        }
        true
    }

    fn emit(&mut self, event: cep47_logic::events::CEP47Event) {
        data::emit(&event)
    }

    fn contact_package_hash(&self) -> casper_types::ContractPackageHash {
        data::contract_package_hash()
    }
}
