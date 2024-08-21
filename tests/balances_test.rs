#[path = "../src/types.rs"]
mod types;

#[path = "../src/balances.rs"]
mod balances;

#[cfg(test)]
mod tests {
    use crate::balances::{self, Pallet};
    use crate::types::{AddressID, Balance};

    fn setup_initial_balance() -> Pallet<AddressID, Balance> {
        let mut balances = balances::Pallet::new();
        let address = AddressID::from("0x123456");
        balances.set_balance(&address, 100);
        return balances;
    }

    #[test]
    pub fn test_balance_set() {
        let mut balances = setup_initial_balance();
        let address = AddressID::from("0x56789");
        balances.set_balance(&address, 200);
        assert_eq!(balances.get_balance(&address), 200);
    }

    #[test]
    pub fn test_balance_get() {
        let balances = setup_initial_balance();
        let address = AddressID::from("0x123456");
        assert_eq!(balances.get_balance(&address), 100);
    }

    #[test]
    pub fn test_default_zero_balance() {
        let balances = setup_initial_balance();
        let fake_address = AddressID::from("0x00000");
        assert_eq!(balances.get_balance(&fake_address), 0);
    }

    #[test]
    pub fn test_transfer_use_case() {
        let mut balances = setup_initial_balance();
        let from_address = AddressID::from("0x12345");
        let to_address = AddressID::from("0x98765");
        balances.set_balance(&from_address, 2000);
        balances.set_balance(&to_address, 0);

        let _ = balances.transfer_balance(from_address.clone(), to_address.clone(), 500);

        assert_eq!(balances.get_balance(&from_address), 1500);
        assert_eq!(balances.get_balance(&to_address), 500);
    }

    #[test]
    pub fn test_insufficient_balance_transfer() {
        let mut balances = setup_initial_balance();
        let from_address = AddressID::from("0x12345");
        let to_address = AddressID::from("0x98765");
        balances.set_balance(&from_address, 1000);
        balances.set_balance(&to_address, 0);

        let transfer = balances.transfer_balance(from_address.clone(), to_address.clone(), 2000);
        assert_eq!(transfer, Err("Insuficient balance!"));
        assert_eq!(balances.get_balance(&from_address), 1000);
        assert_eq!(balances.get_balance(&to_address), 0);
    }
}
