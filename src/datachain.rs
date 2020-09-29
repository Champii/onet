use crate::network_event::NetworkEvent;
use crate::section::Hash;

pub struct DataChain {
    pub data_blocks: Vec<DataBlock>,
}

pub struct DataBlock {
    pub event: NetworkEvent,
    pub signatures: Vec<Hash>,
}
