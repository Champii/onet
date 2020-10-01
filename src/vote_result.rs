use crate::network_event::NetworkEvent;
use crate::section::Hash;

#[derive(Default)]
pub struct VoteResult {
    pub event: Hash,
    pub accept: u8,
    pub deny: u8,
}
