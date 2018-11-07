use std::fmt::{Debug, Formatter, Result as FResult};
use std::net::SocketAddr;
use std::sync::Arc;

use hashgraph::{Key as HgKey, Node as HgNode, NodeConfig, Peer};
use rust_dht::{Dht, DhtConfig, Mutexed, Node as DhtNode, Packet, Plugins, Rpc, Wrapper};

#[derive(Clone, Debug)]
pub struct OnetConfig {
    pub verbose: u8,
    pub listen_addr: SocketAddr,
    pub connect_addr: Option<SocketAddr>,
}

pub struct Onet {
    // key: Key,
    config: OnetConfig,
    dht: Dht,
    hg: HgNode,
    // peers: Peers,
    // hg: Arc<Mutex<Hashgraph>>
}

impl Onet {
    pub fn new(config: OnetConfig) -> Onet {
        let dht_config = DhtConfig {
            verbose: config.verbose,
            listen_addr: config.listen_addr,
            connect_addr: config.connect_addr,
        };

        let mut hg_listen = config.listen_addr.clone();

        hg_listen.set_port(config.listen_addr.port() + 10);

        let hg_config = NodeConfig {
            verbose: config.verbose,
            listen_addr: hg_listen,
        };

        let dht = Dht::new(dht_config);

        let hg_key = HgKey {
            bytes: dht.key.clone().bytes,
            key_pair: dht.key.clone().key_pair,
        };

        Onet {
            config,
            dht: dht,
            hg: HgNode::new(hg_key, hg_config),
        }
    }

    pub fn run(&mut self) {
        // Dont bootstrap
        self.dht.run(false);

        Rpc::Duplex::add_plugin(ConnectWrapper {
            hash: self.dht.hash.clone(),
            pub_key: self.dht.key.get_pub(),
            hg: self.hg.clone(),
        });

        self.dht.bootstrap().unwrap();

        let (tx_in, tx_out) = self.hg.run();

        // tx_in.send("KIK".to_string().into_bytes()).unwrap();

        loop {
            let res = tx_out.recv();

            println!("RESULT {:?}", res);
        }
    }
}

#[derive(Clone)]
struct ConnectWrapper {
    hash: String,
    pub_key: Vec<u8>,
    hg: HgNode,
}

impl Wrapper for ConnectWrapper {
    fn on_recv(&self, pack: &Packet) -> Packet {
        let mut mut_pack = pack.clone();

        let mut d = pack.data.clone();

        // extract the hash
        let mut pub_key = d.split_off(self.hash.len());

        // extract the pubKey
        let data = pub_key.split_off(self.pub_key.len());

        let mut sender = pack.header.sender.clone();

        sender.set_port(sender.port() + 10);

        self.hg
            .peers
            .write()
            .unwrap()
            .add(Peer::new(sender, pub_key.clone()));

        pack.clone()
    }
}

impl Debug for ConnectWrapper {
    fn fmt(&self, fmt: &mut Formatter) -> FResult {
        write!(fmt, "ConnectWrapper")
    }
}
