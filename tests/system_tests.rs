#[path = "../src/types.rs"]
mod types;

#[path = "../src/system.rs"]
mod system;

#[cfg(test)]
mod tests {
    use crate::system::{self, Pallet};
    use crate::types::{AddressID, BlockNumber, Nonce};

    fn system_init() -> Pallet<BlockNumber, AddressID, Nonce> {
        let system = system::Pallet::new();
        return system;
    }

    #[test]
    pub fn test_get_block_number() {
        let system = system_init();
        assert_eq!(system.get_block_number(), BlockNumber::from_be(0));
    }

    #[test]
    pub fn test_get_address_nonce() {
        let address = AddressID::from("alice");
        let system = system_init();
        assert_eq!(system.get_address_nonce(&address), 0);
    }

    #[test]
    pub fn test_increment_block_number() {
        let mut system = system_init();
        system.increment_block_number();
        assert_eq!(system.get_block_number(), 1);
    }

    #[test]
    pub fn test_increment_alice_nonce() {
        let mut system = system_init();
        let address = AddressID::from("alice");
        system.increment_nonce(&address);
        assert_eq!(system.get_address_nonce(&address), 1);
    }
}
