#![allow(dead_code)]
use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

use crate::types;

pub trait Config: types::Config {
    type Balance: CheckedAdd + CheckedSub + Copy + Zero;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AddressID, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, address: &T::AddressID, amount: T::Balance) {
        self.balances.insert(address.clone(), amount);
    }

    pub fn get_balance(&self, address: &T::AddressID) -> T::Balance {
        return *self.balances.get(address).unwrap_or(&T::Balance::zero());
    }

    pub fn transfer_balance(
        &mut self,
        from_address: T::AddressID,
        to_address: T::AddressID,
        amount: T::Balance,
    ) -> types::DispatchResult {
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
