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
}
