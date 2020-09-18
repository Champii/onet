use bincode::serialize;
pub use hashgraph::{Key, Node as HgNode, NodeConfig};
use hex::encode;
use rsrpc::Network;
use rsrpc::TcpTransport;
use rsrpc::Transport;
use rsrpc::UdpTransport;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

use super::rpc::{ClientManagerRpc, DataManagerRpc, RoutingRpc};
use crate::onet::OnetConfig;
// use rust_dht::routing::Routing;

pub struct Client {
    pub node: Node,
}

//

pub struct Node {
    dht_addr: String,
}

pub struct Storage {
    pub path: PathBuf,
}

impl Storage {
    pub fn load() -> Self {
        let mut path = PathBuf::from("/home/champii/.onet");

        Self { path }
    }

    pub fn store(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        let hash = hash(&data);

        let mut path = self.path.clone();

        path.push(encode(hash.clone()));

        std::fs::write(path.clone(), data).or(Err("Error".to_string()))?;

        Ok(hash)
    }

    pub fn get(&self, hash: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut path = self.path.clone();

        path.push(encode(hash));

        std::fs::read(path.clone()).or(Err("Error".to_string()))
    }
}

pub fn hash(item: &Vec<u8>) -> Vec<u8> {
    let serie = serialize(item).unwrap();

    let mut sha = Sha256::default();

    sha.input(serie.as_slice());

    sha.result().to_vec()
}

pub struct Section {
    pub id: i32,
    pub hg: Arc<RwLock<HgNode>>,
    pub members: Vec<String>,
    pub config: OnetConfig,
    pub tx_send: Sender<String>,
    pub storage: Storage,
    // pub rpc: SectionRpc,
}

impl Section {
    pub fn new(config: OnetConfig) -> Self {
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
        }
    }

    pub fn join(&mut self) {}

    pub fn run(&mut self) {
        let mut net = Network::<TcpTransport>::new_default(&self.config.listen_addr);

        net.listen();

        let tx_out = self.hg.write().unwrap().run_with_network(net.clone());

        let mut clientmanager_server = ClientManagerRpc::listen_with_network(net.clone());
        clientmanager_server.context.lock().unwrap().hg = Some(Arc::clone(&self.hg));

        let mut section_server = DataManagerRpc::listen_with_network(net);
        let mut routing_server = RoutingRpc::listen_with_network(net);
        let mut vault_server = VaultRpc::listen_with_network(net);

        // // self.hg.add_tx("KIK".to_string().into_bytes());

        // let net_clone = net.clone();
        // let mut hg = self.hg.clone();

        // let config = self.config.clone();

        // // let mut handle = thread::spawn(move || {
        loop {
            let out = tx_out.recv().unwrap();
            let out = out.as_slice();
            let event: NetworkEvent = bincode::deserialize(&out).unwrap();
            info!("Event: {:?}", event);

            // hg.add_tx(line.unwrap().into_bytes());
        }
        // });

        // handle.join();
        clientmanager_server.wait();
    }
}

pub struct DataChain {
    pub data_blocks: Vec<DataBlock>,
}

pub struct DataBlock {
    pub event: NetworkEvent,
    pub signatures: Vec<Hash>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub creator: Hash,
    pub initiator: Hash,
    pub data: NetworkEventData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkEventData {
    Join(Hash),
    Leave(Hash),
    Store(Hash),
    Read(Hash),
    Accept(Box<NetworkEvent>),
    Deny(Box<NetworkEvent>),
    Blame(Hash),
}

pub struct Vault {
    pub identity: Identity,
    pub section: Option<Section>,
    pub storage: Storage,
    pub config: OnetConfig,
    pub socket: Network<TcpTransport>,
}

impl Vault {
    pub fn new(identity: Identity, config: &OnetConfig, socket: Network<TcpTransport>) -> Self {
        Self {
            identity,
            socket,
            config: config.clone(),
            section: None,
            storage: Storage::load(),
        }
    }

    pub fn connect_bootstrap(&mut self) {
        let routing_client = RoutingRpc::connect_with_network(self.socket.clone());

        let response = routing_client.bootstrap_vault();

        println!("CONNECT BOOTSTRAP RES {:#?}", response);
        // If response OK
        // create section and connect to hg with section routing
    }
}

pub struct Routing {
    pub own_section_hash: Hash,
    pub section_members: HashMap<Hash, Identity>,
    pub sibling_elders: HashMap<Hash, Identity>, // section elders from sibling sections -> identity
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub pub_key: Vec<u8>, // Key ?
    pub cur_ident: Hash,  // Key ?
    pub listening_addr: String,
}

impl Identity {
    pub fn load(config: &OnetConfig) -> Self {
        // TODO: put that in config
        let path = PathBuf::from("~/.onet/identity");

        let res = std::fs::read(path.clone())
            .or(Err("Error".to_string()))
            .unwrap();

        bincode::deserialize(&res).unwrap()
    }
}

pub type Hash = Vec<u8>;
pub type Data = Vec<u8>;
