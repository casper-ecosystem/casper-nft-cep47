#![no_main]

use contract::contract_api::{account, runtime, system};
use types::{runtime_args, ContractHash, PublicKey, RuntimeArgs, U512};

#[no_mangle]
pub extern "C" fn call() {
    let seller: PublicKey = runtime::get_named_arg("seller");
    let marketplace_contract: ContractHash = runtime::get_named_arg("marketplace_contract");

    let transport_purse = system::create_purse();
    system::transfer_from_purse_to_purse(
        account::get_main_purse(),
        transport_purse,
        U512::from(1000u128),
        None,
    )
    .unwrap();

    let offer_key: String = runtime::call_contract(
        marketplace_contract,
        "put_on_sale_test",
        runtime_args! {
            "seller" => seller,
        },
    );
    let _: () = runtime::call_contract(
        marketplace_contract,
        "test_buy",
        runtime_args! {
            // "offer_key" => offer_key,
            "purse" => transport_purse
        },
    );
}
