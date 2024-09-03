#![allow(dead_code)]

use support::Dispatch;
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

impl support::Dispatch for Runtime {
    type AddressID = types::AddressID;
    type Call = types::RuntimeCall;

    fn dispatch(&mut self, address_id: Self::AddressID, call: Self::Call) -> types::DispatchResult {
        todo!();
    }
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> types::DispatchResult {
        self.system.increment_block_number();

        let _ = (self.system.get_block_number() == block.header.block_number)
            .then(|| ())
            .ok_or("Invalid Block Number");

        for (index, support::Extrinsic { address, call }) in block.extrinsic.into_iter().enumerate()
        {
            self.system.increment_nonce(&address);
            let _ = self.dispatch(address, call).map_err(|error| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, index, error
                )
            });
        }
        Ok(())
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
