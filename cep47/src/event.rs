use alloc::vec::Vec;
use casper_types::Key;

use crate::TokenId;

pub enum CEP47Event {
    Mint {
        recipient: Key,
        token_ids: Vec<TokenId>,
    },
    Burn {
        owner: Key,
        token_ids: Vec<TokenId>,
    },
    Approve {
        owner: Key,
        spender: Key,
        token_ids: Vec<TokenId>,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    },
    MetadataUpdate {
        token_id: TokenId,
    },
}
