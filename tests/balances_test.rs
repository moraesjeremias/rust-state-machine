#[path = "../src/balances.rs"]
mod balances;

#[cfg(test)]
mod tests {
    use crate::balances::{self, Pallet};

    fn setup_initial_balance() -> Pallet {
        let mut balances = balances::Pallet::new();
        let address = String::from("0x123456");
        balances.set_balance(&address, 100);
        return balances;
    }

    #[test]
    pub fn test_balance_set() {
        let mut balances = setup_initial_balance();
        let address = String::from("0x56789");
        balances.set_balance(&address, 200);
        assert_eq!(balances.get_balance(&address), 200);
    }

    #[test]
    pub fn test_balance_get() {
        let balances = setup_initial_balance();
        let address = String::from("0x123456");
        assert_eq!(balances.get_balance(&address), 100);
    }

    #[test]
    pub fn test_default_zero_balance() {
        let balances = setup_initial_balance();
        let fake_address = String::from("0x00000");
        assert_eq!(balances.get_balance(&fake_address), 0);
    }
}
