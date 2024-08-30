#![allow(dead_code)]
mod balances;
mod support;
mod system;
mod types;

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self>,
}
impl types::Config for Runtime {
    type AddressID = types::AddressID;
}
impl system::Config for Runtime {
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let address = String::from("alice");
    let bob_address = String::from("bob");
    let charlie_address = String::from("charlie");

    runtime.balances.set_balance(&address, 100);
    runtime.system.increment_block_number();
    assert_eq!(runtime.system.get_block_number(), 1);

    runtime.system.increment_nonce(&address.clone());

    let _transfer_result = runtime
        .balances
        .transfer_balance(address.clone(), bob_address, 30)
        .map_err(|error| eprintln!("Error on transfer funds: {:?}", error));

    runtime.system.increment_nonce(&address.clone());

    let _second_transfer_result = runtime
        .balances
        .transfer_balance(address.clone(), charlie_address, 20)
        .map_err(|error| eprintln!("Error on transfer funds: {:?}", error));
    runtime.balances.get_balance(&address.clone());
    println!("{:#?}", runtime);
}
