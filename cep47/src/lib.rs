#![no_std]
#[macro_use]
extern crate alloc;

mod cep47;
pub mod data;
pub mod event;

pub use cep47::{CEP47, Error};
pub use contract_utils;

use alloc::{collections::BTreeMap, string::String};
pub type TokenId = String;
pub type Meta = BTreeMap<String, String>;
