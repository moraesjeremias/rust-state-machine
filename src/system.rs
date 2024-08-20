#![allow(dead_code)]
use std::collections::BTreeMap;

type AddressID = String;
type BlockNumber = u32;
type Nonce = u32;
type NonceMap = BTreeMap<AddressID, Nonce>;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: NonceMap,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: NonceMap::new(),
        }
    }

    pub fn get_block_number(&self) -> BlockNumber {
        return self.block_number;
    }

    pub fn get_address_nonce(&self, address: &AddressID) -> Nonce {
        return *self.nonce.get(address).unwrap_or(&0);
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn increment_nonce(&mut self, address: &AddressID) {
        let nonce = self.nonce.get(address).unwrap_or(&0);
        let new_nonce = nonce.checked_add(1).unwrap();
        self.nonce.insert(address.clone(), new_nonce);
    }
}
