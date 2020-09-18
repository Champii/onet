use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;
use std::{thread, time};

use super::section::{NetworkEvent, Storage};

service! {
    SectionRpc {
        let storage: Arc<std::sync::RwLock<crate::section::Storage>> = Arc::new(std::sync::RwLock::new(crate::section::Storage::load()));

        fn get_data(&mut self, hash: Vec<u8>) -> Result<Vec<u8>, String> {
            info!("Section: get_data   {}", hex::encode(hash.clone()));

            self.storage.read().unwrap().get(hash)
        }

        fn store_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
            info!("Section: store_data {}", hex::encode(crate::section::hash(&data)));

            self.storage.write().unwrap().store(data)
        }
    }

    ClientManagerRpc {
        let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;
        // let pub_key: Vec<u8> = Vec::new();

        fn get_data(&mut self, hash: Vec<u8>) -> Result<Vec<u8>, String> {
            info!("ClientManager: get_data   {}", hex::encode(hash.clone()));

            let event = crate::section::NetworkEvent {
                creator: vec![],
                // creator: self.pub_key.clone(),
                initiator: vec![],
                data: crate::section::NetworkEventData::Read(hash),
            };

            let serie = bincode::serialize(&event).unwrap();

            if let Some(ref mut hg) = self.hg {
                hg.write().unwrap().add_tx(serie);
            }

            Ok(vec![])
        }

        fn store_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
            let hash = crate::section::hash(&data);
            info!("ClientManager: store_data {}", hex::encode(hash.clone()));

            let event = crate::section::NetworkEvent {
                creator: vec![],
                // creator: self.pub_key.clone(),
                initiator: vec![],
                data: crate::section::NetworkEventData::Store(hash.clone()),
            };

            let serie = bincode::serialize(&event).unwrap();

            if let Some(ref mut hg) = self.hg {
                hg.write().unwrap().add_tx(serie);
            }

            Ok(hash)
        }
    }
}
