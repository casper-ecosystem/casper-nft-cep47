#![allow(dead_code)]
#![allow(unused_imports)]
use types::{AsymmetricType, PublicKey, URef, U256};

pub type TokenId = String;
pub type URI = String;

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

    fn uri(&self) -> URI {
        self.storage().uri()
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

    fn token_uri(&self, token_id: TokenId) -> Option<URI> {
        self.storage().token_uri(token_id)
    }

    fn tokens(&self, owner: PublicKey) -> Vec<TokenId> {
        self.storage().get_tokens(owner)
    }

    // Minter function.
    // Guarded by the entrypoint group.
    fn mint_one(&mut self, recipient: PublicKey, token_uri: URI) {
        self.storage_mut()
            .mint_copies(recipient, token_uri, U256::one());
    }

    fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>) {
        self.storage_mut().mint_many(recipient, token_uris);
    }

    fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256) {
        self.storage_mut().mint_copies(recipient, token_uri, count);
    }

    // Transfer functions.
    fn transfer_token(&mut self, sender: PublicKey, recipient: PublicKey, token_id: TokenId) {
        // 1. Load tokens owned by the sender.
        let mut sender_tokens = self.storage().get_tokens(sender);
        // 2. Assert that token_id is in sender_tokens.
        assert!(
            sender_tokens.contains(&token_id),
            "wrong owner of token {}",
            token_id
        );
        // 3. Remove token_id from sender_tokens.
        sender_tokens.retain(|x| x.clone() != token_id);
        self.storage_mut().set_tokens(sender, sender_tokens);

        // 4. Add token_id to the recipient tokens
        let mut recipient_tokens = self.storage().get_tokens(recipient);
        recipient_tokens.push(token_id);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
    }

    fn transfer_many_tokens(
        &mut self,
        sender: PublicKey,
        recipient: PublicKey,
        token_ids: Vec<TokenId>,
    ) {
        let mut sender_tokens = self.storage().get_tokens(sender);
        for token_id in token_ids.iter() {
            assert!(sender_tokens.contains(token_id), "wrong token {}", token_id);
            sender_tokens.retain(|x| x.clone() != token_id.clone());
        }
        let mut recipient_tokens = self.storage().get_tokens(recipient);
        recipient_tokens.append(&mut token_ids.clone());
        self.storage_mut().set_tokens(sender, sender_tokens);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
    }

    fn transfer_all_tokens(&mut self, sender: PublicKey, recipient: PublicKey) {
        let mut sender_tokens = self.storage().get_tokens(sender);
        let mut recipient_tokens = self.storage().get_tokens(recipient);
        recipient_tokens.append(&mut sender_tokens);

        self.storage_mut().set_tokens(sender, sender_tokens);
        self.storage_mut().set_tokens(recipient, recipient_tokens);
    }

    // URef releated function.
    fn detach(&mut self, owner: PublicKey, token_id: TokenId) -> Option<URef> {
        let mut tokens = self.storage().get_tokens(owner);
        if !tokens.contains(&token_id) {
            None
        } else {
            tokens.retain(|x| x != &token_id);
            self.storage_mut().set_tokens(owner, tokens);
            self.storage_mut().new_uref(token_id)
        }
    }

    fn attach(&mut self, token_uref: URef, recipient: PublicKey) {
        let token_id = self.storage_mut().del_uref(token_uref).unwrap();
        let mut tokens = self.storage().get_tokens(recipient);
        tokens.push(token_id);
        self.storage_mut().set_tokens(recipient, tokens);
    }

    fn token_id(&self, token_uref: URef) -> TokenId {
        self.storage().token_id(token_uref).unwrap()
    }
}

pub trait CEP47Storage {
    // Metadata.
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn uri(&self) -> URI;

    // Getters
    fn balance_of(&self, owner: PublicKey) -> U256;
    fn onwer_of(&self, token_id: TokenId) -> Option<PublicKey>;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> Option<URI>;

    // Setters
    fn get_tokens(&self, owner: PublicKey) -> Vec<TokenId>;
    fn set_tokens(&mut self, owner: PublicKey, token_ids: Vec<TokenId>);
    fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>);
    fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256);

    fn new_uref(&mut self, token_id: TokenId) -> Option<URef>;
    fn del_uref(&mut self, token_uref: URef) -> Option<TokenId>;
    fn token_id(&self, token_uref: URef) -> Option<TokenId>;
}

