use std::collections::{BTreeMap, BTreeSet};

use casper_types::{account::AccountHash, ContractPackageHash, Key};

use crate::TokenId;

pub enum CEP47Event {
    MetadataUpdate {
        token_id: TokenId,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    },
    Mint {
        recipient: Key,
        token_ids: Vec<TokenId>,
    },
    Burn {
        owner: Key,
        token_ids: Vec<TokenId>,
    },
}

impl CEP47Event {
    pub fn type_name(&self) -> String {
        match self {
            CEP47Event::MetadataUpdate { token_id: _ } => "cep47_metadata_update",
            CEP47Event::Transfer {
                sender: _,
                recipient: _,
                token_ids: _,
            } => "cep47_transfer_token",
            CEP47Event::Mint {
                recipient: _,
                token_ids: _,
            } => "cep47_mint_one",
            CEP47Event::Burn {
                owner: _,
                token_ids: _,
            } => "cep47_burn_one",
        }
        .to_string()
    }
}
