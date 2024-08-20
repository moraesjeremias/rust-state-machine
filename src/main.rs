#![allow(dead_code)]
mod balances;
mod system;

fn main() {
    let mut balance_pallet = balances::Pallet::new();
    let mut system_pallet = system::Pallet::new();
}
