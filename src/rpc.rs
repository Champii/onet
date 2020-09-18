use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;
use std::{thread, time};

use super::section::{NetworkEvent, Storage};

service! {
    DataManagerRpc {
        let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;
        // let routing

        fn askjoin(&mut self, pubkey: Vec<u8>) -> bool {
            // check if valid postulant
            // send event to hashgraph
            // collect accept/deny
            // take mesure against bad nodes
            // uppon acceptance add to routing
            // send ok + section routing
            // wait for consensus connection (add to whitelist prior hand)
            true
        }
    }

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

    RoutingRpc {
        // let routing

        // gotten by incoming new vault with age 0
        fn bootstrap_vault(&mut self,) -> bool {
            // generate new RuntimeIdentity : hash(ownID + pub_key)
            // find nearest datamanager in routing
            // ask for join
            // wait for response
            // send ok + section routing
            true
        }
    }
}
