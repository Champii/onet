use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;
use std::{thread, time};

use super::network_event::NetworkEvent;
use super::storage::Storage;

service! {
    DataManagerRpc {
        let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;
        let matching: Option<Arc<std::sync::RwLock<crate::async_response_matcher::AsyncResponseMatcher<bool>>>> = None;
        let routing: Arc<std::sync::RwLock<crate::routing::Routing>> = Arc::new(std::sync::RwLock::new(crate::routing::Routing::new(vec![])));
        // let matching
        // let routing

        // gotten from either ClientManager or another DataManager
        fn store(&mut self, data: Vec<u8>) -> bool {
            // check if self is valid recipient (nearest of data)
            // check creator (clientmanager or datamanager) and initiator (client identity)
            // announce on hashgraph the store event with expected storage vault ids
            // collect accept(store) from Vaults
            // Take mesures for non-responsive/deny-answering/bad-behaving nodes
            // send all accepting nodes the store call with actual data
            true
        }

        fn ask_join(&mut self, identity: crate::identity::Identity) -> bool {
            // check if valid postulant
            let own_hash = self.routing.read().unwrap().own_hash.clone();
            let event = crate::network_event::NetworkEvent::ask_join(own_hash, vec![], identity.clone());

            if let Some(hg) = &self.hg {
                let serie = bincode::serialize(&event).unwrap();

                hg.write().unwrap().add_tx(serie);
            }

            if let Some(matching) = &self.matching {
                let (sender, recv) = std::sync::mpsc::channel();
                matching.write().unwrap().add(identity.cur_ident, sender);

                let response = recv.recv();

                return response.unwrap();
            }
            // collect accept/deny
            // take mesure against bad nodes
            // uppon acceptance add to routing
            // send ok + section routing
            // wait for consensus connection (add to whitelist prior hand)
            false
        }
    }
}

service! {
    ClientManagerRpc {
        let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;

        // gotten from a client
        fn store(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
            // check if self is valid clientmanager (nearest of client identity)
            // fetch nearest datamanager from data
            // send store event to datamanager
            // wait for it to respond with success or failure
            Ok(vec![42])
        }
    }
}

service! {
    VaultRpc {
        // let accepted_data
        // let storage

        // gotten from datamanager
        fn store(&mut self, data: Vec<u8>) -> bool {
            // check if good datamanager (creator)
            // check if that specific data was accepted by self from consensus
            // actualy store the data
            true
        }
    }
}

service! {
    RoutingRpc {
        // let routing
        let routing: Arc<std::sync::RwLock<crate::routing::Routing>> = Arc::new(std::sync::RwLock::new(crate::routing::Routing::new(vec![])));

        fn bootstrap_vault(&mut self, identity: crate::identity::Identity) -> std::collections::HashMap<crate::section::Hash, crate::identity::Identity> {
            // generate new RuntimeIdentity : hash(ownID + pub_key)
            let (cur_ident, own_hash, addr) = {
                let mut routing = self.routing.read().unwrap();

                let datamanager = routing.get_nearest_of(identity.cur_ident.clone()).unwrap();

                (datamanager.cur_ident.clone(), routing.own_hash.clone(), datamanager.listening_addr.clone())
            };

            if cur_ident == own_hash {
                // Direct ask

                let mut dm_client = crate::rpc::DataManagerRpc::connect_tcp(&addr).unwrap();

                dm_client.ask_join(identity.clone());
                let mut routing = self.routing.write().unwrap();

                routing.add(identity);
            } else {
                let mut dm_client = crate::rpc::DataManagerRpc::connect_tcp(&addr).unwrap();

                dm_client.ask_join(identity.clone());

                let mut routing = self.routing.write().unwrap();
                routing.add(identity);
                // wait for response
            }

            {
                let mut routing = self.routing.write().unwrap();

                // send ok + section routing
                routing.section_members.clone()
            }
        }
    }
}
