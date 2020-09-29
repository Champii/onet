use crate::identity::Identity;
use crate::section::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub creator: Hash,
    pub initiator: Hash,
    pub data: NetworkEventData,
}

impl NetworkEvent {
    pub fn ask_join(creator: Hash, initiator: Hash, identity: Identity) -> Self {
        Self {
            creator,
            initiator,
            data: NetworkEventData::Join(identity),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkEventData {
    Join(Identity),
    Leave(Hash),
    Store(Hash),
    Read(Hash),
    Accept(Box<NetworkEvent>),
    Deny(Box<NetworkEvent>),
    Blame(Hash),
}
