#![allow(dead_code)]
use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AddressID, Balance> {
    balances: BTreeMap<AddressID, Balance>,
}

impl<AddressID, Balance> Pallet<AddressID, Balance>
where
    AddressID: Ord + Clone,
    Balance: Zero + CheckedAdd + CheckedSub + Copy,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, address: &AddressID, amount: Balance) {
        self.balances.insert(address.clone(), amount);
    }

    pub fn get_balance(&self, address: &AddressID) -> Balance {
        return *self.balances.get(address).unwrap_or(&Balance::zero());
    }

    pub fn transfer_balance(
        &mut self,
        from_address: AddressID,
        to_address: AddressID,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let from_balance = self.get_balance(&from_address);
        let to_balance = self.get_balance(&to_address);

        let new_from_balance = from_balance
            .checked_sub(&amount)
            .ok_or("Insuficient balance!")?;
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow on adding to balance")?;

        self.set_balance(&from_address, new_from_balance);
        self.set_balance(&to_address, new_to_balance);
        return Ok(());
    }
}
