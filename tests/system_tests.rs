#[path = "../src/types.rs"]
mod types;

#[path = "../src/system.rs"]
mod system;

#[cfg(test)]
mod tests {
    use crate::system::{self, Config, Pallet};
    use crate::types::{self, AddressID, BlockNumber, Nonce};
    struct TestConfig;

    impl types::Config for TestConfig {
        type AddressID = AddressID;
    }

    impl Config for TestConfig {
        type BlockNumber = BlockNumber;
        type Nonce = Nonce;
    }

    fn system_init<T: Config>() -> Pallet<TestConfig> {
        let system = system::Pallet::<TestConfig>::new();
        return system;
    }

    #[test]
    pub fn test_get_block_number() {
        let system = system_init::<TestConfig>();
        assert_eq!(system.get_block_number(), BlockNumber::from_be(0));
    }

    #[test]
    pub fn test_get_address_nonce() {
        let address = AddressID::from("alice");
        let system = system_init::<TestConfig>();
        assert_eq!(system.get_address_nonce(&address), 0);
    }

    #[test]
    pub fn test_increment_block_number() {
        let mut system = system_init::<TestConfig>();
        system.increment_block_number();
        assert_eq!(system.get_block_number(), 1);
    }

    #[test]
    pub fn test_increment_alice_nonce() {
        let mut system = system_init::<TestConfig>();
        let address = AddressID::from("alice");
        system.increment_nonce(&address);
        assert_eq!(system.get_address_nonce(&address), 1);
    }
}
