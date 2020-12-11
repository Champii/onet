// use std::collections::HashMap;
// use std::net::SocketAddr;
// use std::sync::mpsc::{channel, Receiver, Sender};
// use std::sync::{Arc, Mutex, RwLock};
// use std::time::SystemTime;
// use std::{thread, time};

// use super::network_event::NetworkEvent;
// use super::storage::Storage;

// service! {
//     DataManagerRpc {
//         let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;
//         let matching: Option<Arc<std::sync::RwLock<crate::async_response_matcher::AsyncResponseMatcher<bool>>>> = None;
//         let routing: Arc<std::sync::RwLock<crate::routing::Routing>> = Arc::new(std::sync::RwLock::new(crate::routing::Routing::new(vec![])));
//         // let matching
//         // let routing

//         // gotten from either ClientManager or another DataManager
//         fn store(&mut self, data: Vec<u8>) -> bool {
//             // check if self is valid recipient (nearest of data)
//             // check creator (clientmanager or datamanager) and initiator (client identity)
//             // announce on hashgraph the store event with expected storage vault ids
//             // collect accept(store) from Vaults
//             // Take mesures for non-responsive/deny-answering/bad-behaving nodes
//             // send all accepting nodes the store call with actual data
//             true
//         }

//         fn ask_join(&mut self, identity: crate::identity::Identity) -> bool {
//             // check if valid postulant
//             let own_hash = self.routing.read().unwrap().own_hash.clone();
//             let event = crate::network_event::NetworkEvent::ask_join(own_hash, vec![], identity.clone());

//             if let Some(hg) = &self.hg {
//                 let serie = bincode::serialize(&event).unwrap();

//                 hg.write().unwrap().add_tx(serie);
//             }

//             if let Some(matching) = &self.matching {
//                 let (sender, recv) = std::sync::mpsc::channel();
//                 matching.write().unwrap().add(identity.cur_ident, sender);

//                 let response = recv.recv();

//                 return response.unwrap();
//             }
//             // collect accept/deny
//             // take mesure against bad nodes
//             // uppon acceptance add to routing
//             // send ok + section routing
//             // wait for consensus connection (add to whitelist prior hand)
//             false
//         }
//     }
// }

// service! {
//     ClientManagerRpc {
//         let hg: Option<Arc<std::sync::RwLock<crate::section::HgNode>>> = None;

//         // gotten from a client
//         fn store(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
//             // check if self is valid clientmanager (nearest of client identity)
//             // fetch nearest datamanager from data
//             // send store event to datamanager
//             // wait for it to respond with success or failure
//             Ok(vec![42])
//         }
//     }
// }

// service! {
//     VaultRpc {
//         // let accepted_data
//         // let storage

//         // gotten from datamanager
//         fn store(&mut self, data: Vec<u8>) -> bool {
//             // check if good datamanager (creator)
//             // check if that specific data was accepted by self from consensus
//             // actualy store the data
//             true
//         }
//     }
// }

use crate::section::HgNode;
use crate::routing_service::RoutingService;
use std::io::Error;
use crate::routing::Routing;
use std::sync::RwLock;
use std::sync::Arc;
use crate::section::Hash;
use std::collections::HashMap;
use crate::identity::Identity;
use tarpc::transport::channel::UnboundedChannel;
use futures::{future, prelude::*};
use std::{
    io,
    net::{IpAddr, SocketAddr},
};
use tarpc::{
    context,
    client,
    server::{self, Channel, Handler},
};
use tokio_serde::formats::Bincode;


#[tarpc::service]
pub trait Service {
    async fn store(data: Vec<u8>) -> bool;
    async fn bootstrap_vault(identity: Identity) -> Result<HashMap<Hash, Identity>, String>;
    async fn ask_join(identity: Identity) -> Result<bool, String>;
}

#[derive(Clone)]
struct ServerContext {
    routing_service: RoutingService,
}


#[tarpc::server]
impl Service for ServerContext {
    async fn store(self, _: context::Context, data: Vec<u8>) -> bool {
       false
    }

    async fn bootstrap_vault(self, _: context::Context, identity: Identity) -> Result<HashMap<Hash, Identity>, String> {
        self.routing_service.bootstrap_vault(identity).await
    }

    async fn ask_join(self, _: context::Context, identity: Identity) -> Result<bool, String> {
        debug!("AskJoin");
        let own_hash = self.routing_service.routing.read().unwrap().own_hash.clone();
        let event = crate::network_event::NetworkEvent::ask_join(own_hash, vec![], identity.clone());

        // if let Some(hg) = &self.hg {
        let serie = bincode::serialize(&event).unwrap();

        self.routing_service.hg.write().unwrap().add_tx(serie);
        // }

        // if let Some(matching) = &self.matching {
        //     let (sender, recv) = std::sync::mpsc::channel();
        //     matching.write().unwrap().add(identity.cur_ident, sender);

        //     let response = recv.recv();

        //     return response.unwrap();
        // }
        // collect accept/deny
        // take mesure against bad nodes
        // uppon acceptance add to routing
        // send ok + section routing
        // wait for consensus connection (add to whitelist prior hand)
        Ok(true)
    }
}

pub async fn listen(server_addr: SocketAddr, routing: Arc<RwLock<Routing>>, hg: Arc<RwLock<HgNode>>) -> io::Result<()> {
    let mut listener =
        tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default).await?;

    listener.config_mut().max_frame_length(4294967296);


    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = ServerContext {
                routing_service: RoutingService {
                    routing: Arc::clone(&routing),
                    hg: Arc::clone(&hg),
                },
            };

            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}

pub async fn client(addr: String) -> ServiceClient {
    let transport = tarpc::serde_transport::tcp::connect(&addr, Bincode::default)
        .await
        .map_err(|e| format!("{}", e)).unwrap();

    ServiceClient::new(client::Config::default(), transport)
        .spawn()
        .map_err(|e| format!("{}", e)).unwrap()
}
