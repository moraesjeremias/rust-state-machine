#![allow(dead_code)]
use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait Config {
    type AddressID: Ord + Clone;
    type BlockNumber: CheckedAdd + Copy + One + Zero;
    type Nonce: CheckedAdd + Copy + One + Zero;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AddressID, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        return self.block_number;
    }

    pub fn get_address_nonce(&self, address: &T::AddressID) -> T::Nonce {
        return *self.nonce.get(address).unwrap_or(&T::Nonce::zero());
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap();
    }

    pub fn increment_nonce(&mut self, address: &T::AddressID) {
        let default_nonce = T::Nonce::zero();
        let nonce = self.nonce.get(address).unwrap_or(&default_nonce);
        let new_nonce = nonce.checked_add(&T::Nonce::one()).unwrap();
        self.nonce.insert(address.clone(), new_nonce);
    }
}
