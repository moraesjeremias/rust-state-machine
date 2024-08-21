#![allow(dead_code)]
use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

#[derive(Debug)]
pub struct Pallet<BlockNumber, AddressID, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AddressID, Nonce>,
}

impl<BlockNumber, AddressID, Nonce> Pallet<BlockNumber, AddressID, Nonce>
where
    AddressID: Ord + Clone,
    BlockNumber: CheckedAdd + Copy + One + Zero,
    Nonce: CheckedAdd + Copy + One + Zero,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn get_block_number(&self) -> BlockNumber {
        return self.block_number;
    }

    pub fn get_address_nonce(&self, address: &AddressID) -> Nonce {
        return *self.nonce.get(address).unwrap_or(&Nonce::zero());
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
    }

    pub fn increment_nonce(&mut self, address: &AddressID) {
        let default_nonce = Nonce::zero();
        let nonce = self.nonce.get(address).unwrap_or(&default_nonce);
        let new_nonce = nonce.checked_add(&Nonce::one()).unwrap();
        self.nonce.insert(address.clone(), new_nonce);
    }
}
