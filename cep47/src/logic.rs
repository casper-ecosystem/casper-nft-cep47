use std::marker::PhantomData;

use contract::contract_api::{runtime, storage};
use types::{U256, PublicKey, URef};

type TokenId = String;
type URI = String;

trait WithStorage<Storage: CEP47Storage> {
    fn storage(&self) -> Storage;
}

trait CEP47Contract<Storage: CEP47Storage>: WithStorage<Storage> {
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
        todo!();
    }
    
    // Minter function.
    // Guarded by the entrypoint group.
    fn mint_one(&mut self, recipient: PublicKey, token_uri: URI) {
        self.storage().mint_copies(recipient, token_uri, U256::one());
    }
    
    fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>) {
        self.storage().mint_many(recipient, token_uris);
    }
    
    fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256) {
        self.storage().mint_copies(recipient, token_uri, count);
    }

    // Transfer functions.
    fn transfer_token(&mut self, sender: PublicKey, recipient: PublicKey, token_id: TokenId) {
        // 1. Load tokens owned by the sender.
        let sender_tokens = self.storage().get_tokens(sender);
        // 2. Assert that token_id is in sender_tokens.
        
        // 3. Remove token_id from sender_tokens.
        let updated_sender_tokens = sender_tokens; // Modify
        self.storage().set_tokens(sender, updated_sender_tokens);
        
        // 4. Add token_id to the recipient tokens 
        let recipient_tokens = self.storage().get_tokens(recipient);
        let updated_recipient_tokens = recipient_tokens;
        self.storage().set_tokens(recipient, updated_recipient_tokens);
    }
    
    fn transfer_many_tokens(&mut self, sender: PublicKey, recipient: PublicKey, token_ids: Vec<TokenId>) {
        todo!()
    }
    fn transfer_all_tokens(&mut self, sender: PublicKey, recipient: PublicKey) {
        todo!()
    }

    // URef releated function.    
    fn detach(&mut self, owner: PublicKey, token_id: TokenId) -> URef {
        todo!();
    }
    fn attach(&mut self, token_uref: URef, recipient: PublicKey) {}
    fn token_id(&self, token_uref: URef) -> TokenId { todo!(); }
}

trait CEP47Storage {

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
    fn new_uref(&mut self, token_id: TokenId) -> URef;
    fn del_uref(&mut self, token_uref: URef);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::{PublicKey, TokenId};
    use super::{WithStorage, CEP47Storage, CEP47Contract};

    struct TestStorage {
        name: String,
        tokens: BTreeMap<PublicKey, Vec<TokenId>>
    }

    impl TestStorage {
        pub fn new() -> TestStorage {
            TestStorage {
                name: String::from("asd"),
                tokens: BTreeMap::new()
            }
        }
    }

    impl CEP47Storage for TestStorage {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn symbol(&self) -> String {
        todo!()
    }

        fn uri(&self) -> super::URI {
        todo!()
    }

        fn balance_of(&self, owner: types::PublicKey) -> types::U256 {
        todo!()
    }

        fn onwer_of(&self, token_id: super::TokenId) -> Option<types::PublicKey> {
        todo!()
    }

        fn total_supply(&self) -> types::U256 {
        todo!()
    }

        fn token_uri(&self, token_id: super::TokenId) -> Option<super::URI> {
        todo!()
    }

        fn get_tokens(&self, owner: types::PublicKey) -> Vec<super::TokenId> {
        todo!()
    }

        fn set_tokens(&mut self, owner: types::PublicKey, token_ids: Vec<super::TokenId>) {
        todo!()
    }

        fn mint_many(&mut self, recipient: types::PublicKey, token_uris: Vec<super::URI>) {
        todo!()
    }

        fn mint_copies(&mut self, recipient: types::PublicKey, token_uri: super::URI, count: types::U256) {
        todo!()
    }

        fn new_uref(&mut self, token_id: super::TokenId) -> types::URef {
        todo!()
    }

        fn del_uref(&mut self, token_uref: types::URef) {
        todo!()
    }
    }

    struct TestContract {}

    impl WithStorage<TestStorage> for TestContract {
        fn storage(&self) -> TestStorage {
            TestStorage::new()
        }
    }

    impl CEP47Contract<TestStorage> for TestContract{}

    #[test]
    fn test_name() {
        let contract = TestContract {};
        assert_eq!(contract.name(), String::from("asd"))
    }

}