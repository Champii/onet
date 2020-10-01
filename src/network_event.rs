use crate::identity::Identity;
use crate::section::Hash;
use std::fmt::Display;

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

    pub fn accept(creator: Hash, initiator: Hash, identity: Identity) -> Self {
        Self {
            creator,
            initiator,
            data: NetworkEventData::Accept(identity),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkEventData {
    Join(Identity),
    Leave(Hash),
    Store(Hash),
    Read(Hash),
    Accept(Identity),
    Deny(Box<NetworkEvent>),
    Blame(Hash),
}

impl Display for NetworkEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Display for NetworkEventData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NetworkEventData::Join(_) => "Join",
                NetworkEventData::Accept(_) => "Accept",
                _ => "TODO",
            }
        )
    }
}
