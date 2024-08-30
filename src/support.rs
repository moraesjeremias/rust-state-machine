use crate::types;

pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsic: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<AddressID, Call> {
    pub address: AddressID,
    pub call: Call,
}

pub trait Dispatch {
    type AddressID;
    type Call;

    fn dispatch(&mut self, address_id: Self::AddressID, call: Self::Call) -> types::DispatchResult;
}
