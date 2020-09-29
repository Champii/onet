use crate::network_event::NetworkEvent;

pub struct VoteResult {
    event: NetworkEvent,
    accept: Vec<NetworkEvent>,
    deny: Vec<NetworkEvent>,
}
