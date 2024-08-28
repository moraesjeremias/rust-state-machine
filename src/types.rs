#![allow(dead_code)]
pub type AddressID = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;

pub trait Config {
    type AddressID: Ord + Clone;
}
