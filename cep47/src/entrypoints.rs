use std::collections::BTreeSet;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    CLType, CLTyped, ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, Parameter,
};
use cep47_logic::{Meta, TokenId};

pub fn get_entrypoints(package_hash: Option<ContractPackageHash>) -> EntryPoints {
    let secure = if let Some(contract_package_hash) = package_hash {
        let deployer_group = storage::create_contract_user_group(
            contract_package_hash,
            "deployer",
            1,
            BTreeSet::default(),
        )
        .unwrap_or_revert();
        runtime::put_key("deployer_group_access", Key::URef(deployer_group[0]));
        true
    } else {
        false
    };

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String, None));
    entry_points.add_entry_point(endpoint("meta", vec![], Meta::cl_type(), None));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256, None));
    entry_points.add_entry_point(endpoint("is_paused", vec![], CLType::Bool, None));
    entry_points.add_entry_point(endpoint(
        "pause",
        vec![],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "unpause",
        vec![],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", CLType::Key)],
        CLType::U256,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::String)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "total_supply",
        vec![Parameter::new("owner", CLType::Key)],
        CLType::List(Box::new(CLType::String)),
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "mint_one",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new("token_ids", CLType::Option(Box::new(TokenId::cl_type()))),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_many",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_metas", CLType::List(Box::new(Meta::cl_type()))),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "mint_copies",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new(
                "token_ids",
                CLType::Option(Box::new(CLType::List(Box::new(TokenId::cl_type())))),
            ),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "update_token_metadata",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "burn_one",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "burn_many",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        CLType::Unit,
        if secure { Some("deployer") } else { None },
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_token",
        vec![
            Parameter::new("sender", CLType::Key),
            Parameter::new("recipient", CLType::Key),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Unit,
        None,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_many_tokens",
        vec![
            Parameter::new("sender", CLType::Key),
            Parameter::new("recipient", CLType::Key),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        CLType::Unit,
        None,
    ));
    entry_points
}

pub fn endpoint(
    name: &str,
    param: Vec<Parameter>,
    ret: CLType,
    access: Option<&str>,
) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        match access {
            None => EntryPointAccess::Public,
            Some(access_key) => EntryPointAccess::groups(&[access_key]),
        },
        EntryPointType::Contract,
    )
}
