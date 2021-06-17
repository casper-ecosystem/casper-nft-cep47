use crate::{get_key, remove_key, set_key};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use contract::contract_api::runtime::revert;
use contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use core::convert::TryInto;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use types::bytesrepr::Error;
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    AccessRights, ApiError, AsymmetricType, CLType, CLTyped, CLValue, ContractPackageHash,
    EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, PublicKey, URef,
    U256, U512,
};

#[derive(Clone)]
pub struct Offer {
    pub seller: PublicKey,
    pub price: U512,
    pub item: URef,
    pub designation: String,
}

impl ToBytes for Offer {
    /// Serializes `&self` to a `Vec<u8>`.
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut ret = Vec::new();
        ret.extend(self.seller.to_bytes().unwrap());
        ret.extend(self.price.to_bytes().unwrap());
        ret.extend(self.item.to_bytes().unwrap());
        ret.extend(self.designation.to_bytes().unwrap());
        Ok(ret)
    }
    /// Consumes `self` and serializes to a `Vec<u8>`.
    fn into_bytes(self) -> Result<Vec<u8>, Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
    fn serialized_length(&self) -> usize {
        self.seller.serialized_length()
            + self.price.serialized_length()
            + self.item.serialized_length()
            + self.designation.serialized_length()
    }
}

impl FromBytes for Offer {
    /// Deserializes the slice into `Self`.
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let pk = PublicKey::from_bytes(&bytes[0..33]).unwrap().0;
        let price = U512::from_bytes(&bytes[33..36]).unwrap().0;
        let uref = URef::from_bytes(&bytes[36..69]).unwrap().0;
        let name = String::from_bytes(&bytes[69..]).unwrap().0;
        Ok((Offer::new(pk, price, uref, name), &[]))
    }
    /// Deserializes the `Vec<u8>` into `Self`.
    fn from_vec(bytes: Vec<u8>) -> Result<(Self, Vec<u8>), Error> {
        Self::from_bytes(bytes.as_slice()).map(|(x, remainder)| (x, Vec::from(remainder)))
    }
}

impl CLTyped for Offer {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl Offer {
    pub fn new(seller: PublicKey, price: U512, item: URef, designation: String) -> Self {
        Self {
            seller,
            price,
            item,
            designation,
        }
    }

    pub fn store(&self) -> String {
        let key = self.offer_key("");
        set_key(&key, self.clone());
        key
    }

    pub fn load(key: &str) -> Offer {
        get_key(key).unwrap_or_revert()
    }
    pub fn offer_key(&self, flag: &str) -> String {
        format!("{}_{}{}", self.seller.to_hex(), self.designation, flag)
    }

    pub fn test_struct(seller: PublicKey) -> Self {
        Self {
            seller: seller,
            price: U512::from(1000),
            item: storage::new_uref("test"),
            designation: "test_order".to_string(),
        }
    }
}
