#![allow(dead_code)]
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn get_block_number(&self) -> u32 {
        return self.block_number;
    }

    pub fn get_address_nonce(&self, address: &String) -> u32 {
        return *self.nonce.get(address).unwrap_or(&0);
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn increment_nonce(&mut self, address: &String) {
        let nonce = self.nonce.get(address).unwrap_or(&0);
        let new_nonce = nonce.checked_add(1).unwrap();
        self.nonce.insert(address.clone(), new_nonce);
    }
}
