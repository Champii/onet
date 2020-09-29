use crate::async_response_matcher::AsyncResponseMatcher;
use crate::async_vote_matcher::AsyncVoteMatcher;
use crate::network_event::{NetworkEvent, NetworkEventData};
use crate::routing::Routing;
use crate::storage::Storage;
pub use hashgraph::{Key, Node as HgNode, NodeConfig};
use rsrpc::Network;
use rsrpc::TcpTransport;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

use super::rpc::{ClientManagerRpc, DataManagerRpc, RoutingRpc, VaultRpc};
use crate::onet::OnetConfig;

pub type Hash = Vec<u8>;
pub type Data = Vec<u8>;

pub struct Section {
    pub id: i32,
    pub hg: Arc<RwLock<HgNode>>,
    pub members: Vec<String>,
    pub config: OnetConfig,
    pub tx_send: Sender<String>,
    pub storage: Storage,
    // pub rpc: SectionRpc,
    pub routing: Arc<RwLock<Routing>>,
}

impl Section {
    pub fn new(config: OnetConfig, routing: Routing) -> Self {
        let (tx_send, tx_recv) = channel();

        let hg_config = NodeConfig {
            verbose: config.verbose,
            listen_addr: config.listen_addr.clone(),
            connect_addr: config.connect_addr,
        };

        let hg = HgNode::new(Key::new_generate().unwrap(), hg_config);

        Self {
            id: 0,
            hg: Arc::new(RwLock::new(hg)),
            members: vec![],
            config,
            tx_send,
            storage: Storage::load(),
            routing: Arc::new(RwLock::new(routing)),
        }
    }

    // pub fn join(&mut self) {}

    pub fn run(&mut self, net: Network<TcpTransport>) {
        // let mut net = Network::<TcpTransport>::new_default(&self.config.listen_addr);

        // net.listen();

        let tx_out = self.hg.write().unwrap().run_with_network(net.clone());

        let mut clientmanager_server = ClientManagerRpc::listen_with_network(net.clone());
        clientmanager_server.context.lock().unwrap().hg = Some(Arc::clone(&self.hg));

        let mut datamanager_server = DataManagerRpc::listen_with_network(net.clone());
        datamanager_server.context.lock().unwrap().hg = Some(Arc::clone(&self.hg));

        let mut routing_server = RoutingRpc::listen_with_network(net.clone());
        routing_server.context.lock().unwrap().routing = Arc::clone(&self.routing);

        let mut vault_server = VaultRpc::listen_with_network(net.clone());

        let voteMatcher = AsyncVoteMatcher::new();
        let asyncMatcher = AsyncResponseMatcher::new();

        let own_hash = self.routing.read().unwrap().own_hash.clone();

        let mut handle = thread::spawn(move || {
            loop {
                let out = tx_out.recv().unwrap();
                let out = out.as_slice();

                let event: NetworkEvent = bincode::deserialize(&out).unwrap();

                // if event should receive votes,
                // add to async matcher
                // add to async vote receiver
                // on sufficient votes, trigger matcher with result

                info!("Event: {:?}", event);

                match event.data {
                    NetworkEventData::Join(hash) => {
                        if event.creator == own_hash {
                            // is own event so dont cast vote ?
                        } else {
                            // cast vote
                        }
                    }
                    NetworkEventData::Accept(hash) => if event.creator == own_hash {},
                    _ => (),
                }
                // hg.add_tx(line.unwrap().into_bytes());
            }
        });

        handle.join();
        clientmanager_server.wait();
    }
}
