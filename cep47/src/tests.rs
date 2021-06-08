#[cfg(test)]
mod tests {
    use casper_types::AccessRights;
    use rand::Rng;

    use crate::{
        AsymmetricType, CEP47Contract, CEP47Storage, PublicKey, TokenId, URef, WithStorage, U256,
        URI,
    };
    use std::{
        collections::{hash_map::DefaultHasher, BTreeMap},
        hash::{Hash, Hasher},
        sync::Mutex,
    };

    struct TestStorage {
        name: String,
        symbol: String,
        uri: URI,
        total_supply: U256,
        tokens: BTreeMap<PublicKey, Vec<TokenId>>,
        token_uris: BTreeMap<TokenId, URI>,
        balances: BTreeMap<PublicKey, U256>,
        belongs_to: BTreeMap<TokenId, PublicKey>,
        urefs: BTreeMap<URef, TokenId>,
    }

    impl TestStorage {
        pub fn new() -> TestStorage {
            TestStorage {
                name: String::from("Casper Enhancement Proposal 47"),
                symbol: String::from("CEP47"),
                uri: URI::from("https://github.com/casper-ecosystem/casper-nft-cep47"),
                total_supply: U256::from(0),
                tokens: BTreeMap::new(),
                balances: BTreeMap::new(),
                belongs_to: BTreeMap::new(),
                token_uris: BTreeMap::new(),
                urefs: BTreeMap::new(),
            }
        }
    }

    impl CEP47Storage for TestStorage {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn symbol(&self) -> String {
            self.symbol.clone()
        }

        fn uri(&self) -> URI {
            self.uri.clone()
        }

        fn balance_of(&self, owner: PublicKey) -> U256 {
            let owner_balance = self.balances.get(&owner);
            if owner_balance.is_none() {
                U256::from(0)
            } else {
                owner_balance.unwrap().clone()
            }
        }

        fn onwer_of(&self, token_id: TokenId) -> Option<PublicKey> {
            let owner = self.belongs_to.get(&token_id);
            if owner.is_some() {
                Some(owner.unwrap().clone())
            } else {
                None
            }
        }

        fn total_supply(&self) -> U256 {
            self.total_supply
        }

        fn token_uri(&self, token_id: TokenId) -> Option<URI> {
            let uri = self.token_uris.get(&token_id);
            if uri.is_some() {
                Some(uri.unwrap().clone())
            } else {
                None
            }
        }

        fn get_tokens(&self, owner: PublicKey) -> Vec<TokenId> {
            let owner_tokens = self.tokens.get(&owner);
            if owner_tokens.is_none() {
                Vec::<TokenId>::new()
            } else {
                owner_tokens.unwrap().clone()
            }
        }

        fn set_tokens(&mut self, owner: PublicKey, token_ids: Vec<TokenId>) {
            let owner_new_balance = U256::from(token_ids.len() as u64);

            let owner_tokens = self.get_tokens(owner.clone());
            for token_id in owner_tokens.clone() {
                self.belongs_to.remove(&token_id);
            }
            for token_id in token_ids.clone() {
                self.belongs_to.insert(token_id, owner.clone());
            }

            self.tokens.insert(owner.clone(), token_ids.clone());
            self.balances.insert(owner, owner_new_balance);
        }

        fn mint_many(&mut self, recipient: PublicKey, token_uris: Vec<URI>) {
            let recipient_balance = self.balances.get(&recipient);
            let recipient_tokens = self.tokens.get(&recipient);
            let mut recipient_new_balance = if recipient_balance.is_none() {
                U256::from(0)
            } else {
                recipient_balance.unwrap().clone()
            };
            let mut recipient_new_tokens = if recipient_tokens.is_none() {
                Vec::<TokenId>::new()
            } else {
                recipient_tokens.unwrap().clone()
            };

            let mut hasher = DefaultHasher::new();

            for token_uri in token_uris.clone() {
                let token_info = (self.total_supply, self.uri.clone(), token_uri.clone());
                Hash::hash(&token_info, &mut hasher);
                let token_id: TokenId = TokenId::from(hasher.finish().to_string());
                self.token_uris.insert(token_id.clone(), token_uri);
                recipient_new_tokens.push(token_id.clone());
                self.belongs_to.insert(token_id, recipient.clone());
                recipient_new_balance = recipient_new_balance + 1;
                self.total_supply = self.total_supply + 1;
            }
            self.balances
                .insert(recipient.clone(), recipient_new_balance);
            self.tokens.insert(recipient, recipient_new_tokens);
        }

        fn mint_copies(&mut self, recipient: PublicKey, token_uri: URI, count: U256) {
            let token_uris: Vec<URI> = vec![token_uri; count.as_usize()];
            self.mint_many(recipient, token_uris);
        }

        fn new_uref(&mut self, token_id: TokenId) -> Option<URef> {
            let mut rng = rand::thread_rng();
            let val: [u8; 32] = rng.gen();
            let uref = URef::new(val, AccessRights::READ_ADD_WRITE);
            if self.urefs.contains_key(&uref) {
                None
            } else {
                self.urefs.insert(uref, token_id);
                Some(uref)
            }
        }

        fn del_uref(&mut self, token_uref: URef) -> Option<TokenId> {
            let token_id = self.token_id(token_uref);
            if token_id.is_none() {
                None
            } else {
                let token_id = token_id.unwrap();
                self.urefs.remove(&token_uref);
                Some(token_id)
            }
        }

        fn token_id(&self, token_uref: URef) -> Option<TokenId> {
            self.urefs.get(&token_uref).map(|x| x.clone())
        }
    }

    struct TestContract {
        storage: TestStorage,
    }

    impl TestContract {
        pub fn new() -> TestContract {
            TestContract {
                storage: TestStorage::new(),
            }
        }
    }

    impl WithStorage<TestStorage> for TestContract {
        fn storage(&self) -> &TestStorage {
            &self.storage
        }

        fn storage_mut(&mut self) -> &mut TestStorage {
            &mut self.storage
        }
    }

    impl CEP47Contract<TestStorage> for TestContract {}

    #[test]
    fn test_metadata() {
        let contract = TestContract::new();
        assert_eq!(
            contract.name(),
            String::from("Casper Enhancement Proposal 47")
        );
        assert_eq!(contract.symbol(), String::from("CEP47"));
        assert_eq!(
            contract.uri(),
            String::from("https://github.com/casper-ecosystem/casper-nft-cep47")
        );
    }
    #[test]
    fn test_mint_many() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        assert_eq!(contract.total_supply(), U256::from(0));
        contract.mint_many(ali.clone(), vec![URI::from("Apple URI")]);
        contract.mint_many(
            bob.clone(),
            vec![URI::from("Banana URI"), URI::from("Orange URI")],
        );
        assert_eq!(contract.total_supply(), U256::from(3));

        let ali_balance = contract.balance_of(ali.clone());
        assert_eq!(ali_balance, U256::from(1));
        let bob_balance = contract.balance_of(bob.clone());
        assert_eq!(bob_balance, U256::from(2));

        let ali_tokens: Vec<TokenId> = contract.tokens(ali);
        let ali_first_token_uri: URI = contract
            .token_uri(ali_tokens.get(0).unwrap().clone())
            .unwrap();
        assert_eq!(ali_first_token_uri, URI::from("Apple URI"));

        let bob_tokens: Vec<TokenId> = contract.tokens(bob);
        let bob_first_token_uri: URI = contract
            .token_uri(bob_tokens.get(1).unwrap().clone())
            .unwrap();
        assert_eq!(bob_first_token_uri, URI::from("Orange URI"));
    }
    #[test]
    fn test_mint_copies() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();

        assert_eq!(contract.total_supply(), U256::from(0));
        contract.mint_copies(ali.clone(), URI::from("Casper Fan URI"), U256::from(7));
        assert_eq!(contract.total_supply(), U256::from(7));

        let ali_balance = contract.balance_of(ali.clone());
        assert_eq!(ali_balance, U256::from(7));

        let ali_tokens: Vec<TokenId> = contract.tokens(ali);
        let ali_first_token_uri: URI = contract
            .token_uri(ali_tokens.get(0).unwrap().clone())
            .unwrap();
        let ali_third_token_uri: URI = contract
            .token_uri(ali_tokens.get(2).unwrap().clone())
            .unwrap();
        assert_eq!(ali_first_token_uri, URI::from("Casper Fan URI"));
        assert_eq!(ali_third_token_uri, URI::from("Casper Fan URI"));
    }
    #[test]
    fn test_transfer_token() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        assert_eq!(contract.total_supply(), U256::from(0));
        contract.mint_one(ali.clone(), URI::from("Casper Fan URI"));
        assert_eq!(contract.total_supply(), U256::from(1));

        let mut ali_balance = contract.balance_of(ali.clone());
        let mut bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(1));
        assert_eq!(bob_balance, U256::from(0));

        let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
        let ali_first_token_id: TokenId = ali_tokens.get(0).unwrap().clone();
        let ali_first_token_uri: URI = contract.token_uri(ali_first_token_id.clone()).unwrap();
        assert_eq!(ali_first_token_uri, URI::from("Casper Fan URI"));

        let transfer_res =
            contract.transfer_token(ali.clone(), bob.clone(), ali_first_token_id.clone());
        assert!(transfer_res.is_ok());
        ali_balance = contract.balance_of(ali);
        bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(0));
        assert_eq!(bob_balance, U256::from(1));

        let owner_of_first_token_id = contract.owner_of(ali_first_token_id);
        assert_eq!(owner_of_first_token_id.unwrap(), bob);
    }
    #[test]
    fn test_transfer_all_tokens() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        assert_eq!(contract.total_supply(), U256::from(0));
        contract.mint_many(
            ali.clone(),
            vec![URI::from("Apple URI"), URI::from("Banana URI")],
        );
        contract.mint_one(ali.clone(), URI::from("Casper Fan URI"));
        assert_eq!(contract.total_supply(), U256::from(3));

        let mut ali_balance = contract.balance_of(ali.clone());
        let mut bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(3));
        assert_eq!(bob_balance, U256::from(0));

        let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
        let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
        let ali_second_token_uri: URI = contract.token_uri(ali_second_token_id.clone()).unwrap();
        assert_eq!(ali_second_token_uri, URI::from("Banana URI"));

        contract.transfer_all_tokens(ali.clone(), bob.clone());

        ali_balance = contract.balance_of(ali);
        bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(0));
        assert_eq!(bob_balance, U256::from(3));

        let owner_of_second_token_id = contract.owner_of(ali_second_token_id);
        assert_eq!(owner_of_second_token_id.unwrap(), bob);
    }

    #[test]
    fn test_transfer_many_tokens() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        assert_eq!(contract.total_supply(), U256::from(0));
        contract.mint_many(
            ali.clone(),
            vec![URI::from("Apple URI"), URI::from("Banana URI")],
        );
        contract.mint_copies(ali.clone(), URI::from("Casper Fan URI"), U256::from(3));
        assert_eq!(contract.total_supply(), U256::from(5));

        let mut ali_balance = contract.balance_of(ali.clone());
        let mut bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(5));
        assert_eq!(bob_balance, U256::from(0));

        let ali_tokens: Vec<TokenId> = contract.tokens(ali.clone());
        let ali_second_token_id: TokenId = ali_tokens.get(1).unwrap().clone();
        let ali_second_token_uri: URI = contract.token_uri(ali_second_token_id.clone()).unwrap();
        let ali_third_token_id: TokenId = ali_tokens.get(2).unwrap().clone();
        let ali_third_token_uri: URI = contract.token_uri(ali_third_token_id.clone()).unwrap();
        assert_eq!(ali_second_token_uri, URI::from("Banana URI"));
        assert_eq!(ali_third_token_uri, URI::from("Casper Fan URI"));

        let transfer_res = contract.transfer_many_tokens(
            ali.clone(),
            bob.clone(),
            vec![ali_second_token_id.clone(), ali_third_token_id.clone()],
        );
        assert!(transfer_res.is_ok());

        ali_balance = contract.balance_of(ali);
        bob_balance = contract.balance_of(bob.clone());
        assert_eq!(ali_balance, U256::from(3));
        assert_eq!(bob_balance, U256::from(2));

        let owner_of_second_token_id = contract.owner_of(ali_second_token_id);
        let owner_of_third_token_id = contract.owner_of(ali_third_token_id);
        assert_eq!(owner_of_second_token_id.unwrap(), bob);
        assert_eq!(owner_of_third_token_id.unwrap(), bob);
    }

    #[test]
    fn test_attach_and_detach() {
        let mut contract = TestContract::new();
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        contract.mint_one(ali.clone(), URI::from("0x12af"));
        let token_id: TokenId = contract.tokens(ali.clone())[0].clone();

        let token_uref: URef = contract.detach(ali.clone(), token_id.clone()).unwrap();
        assert_eq!(contract.balance_of(ali.clone()), U256::zero());
        assert_eq!(contract.total_supply(), U256::one());
        assert!(contract.tokens(ali).is_empty());

        assert_eq!(contract.token_id(token_uref.clone()), token_id.clone());
        assert_eq!(
            contract.token_uri(token_id.clone()).unwrap(),
            URI::from("0x12af")
        );

        contract.attach(token_uref, bob.clone());
        assert_eq!(contract.balance_of(bob.clone()), U256::one());
        assert_eq!(contract.total_supply(), U256::one());
        assert_eq!(contract.tokens(bob), vec![token_id]);
    }
}
