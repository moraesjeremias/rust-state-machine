#![allow(dead_code)]
use crate::support;

pub type AddressID = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type DispatchResult = Result<(), &'static str>;
pub type Extrinsic = support::Extrinsic<AddressID, RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;

pub trait Config {
    type AddressID: Ord + Clone;
}

pub enum RuntimeCall {}
