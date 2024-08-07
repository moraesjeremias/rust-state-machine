use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, address: &String, amount: u128) {
        self.balances.insert(address.clone(), amount);
    }

    pub fn get_balance(&self, address: &String) -> u128 {
        *self.balances.get(address).unwrap_or(&0)
    }

}
