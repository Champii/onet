use bincode::serialize;
pub use hashgraph::{Key, Node as HgNode, NodeConfig};
use hex::encode;
use rsrpc::Network;
use rsrpc::TcpTransport;
use rsrpc::Transport;
use rsrpc::UdpTransport;
use sha2::{Digest, Sha256};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

use super::rpc::{ClientManagerRpc, SectionRpc};
use crate::onet::OnetConfig;

pub struct Vault {
    pub node: Node,
    pub section: Section,
}

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

        let mut section_server = SectionRpc::listen_with_network(net);

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
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub creator: Vec<u8>,
    pub initiator: Vec<u8>,
    pub data: NetworkEventData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkEventData {
    Join(Vec<u8>),
    Leave(Vec<u8>),
    Store(Vec<u8>),
    Read(Vec<u8>),
    Accept(Box<NetworkEvent>),
    Deny(Box<NetworkEvent>),
    Blame(Vec<u8>),
}
