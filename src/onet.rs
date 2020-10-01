use crate::identity::Identity;
use crate::vault::Vault;
use rsrpc::Network;
use rsrpc::TcpTransport;
use std::net::SocketAddr;

use hashgraph::{Key, Node as HgNode, NodeConfig};
// use rust_dht::{Dht, DhtConfig, Packet, Rpc, Wrapper};

#[derive(Clone, Debug)]
pub enum OnetMode {
    Client,
    Vault,
}

#[derive(Clone, Debug)]
pub struct OnetConfig {
    pub verbose: u8,
    pub listen_addr: SocketAddr,
    pub connect_addr: Option<SocketAddr>,
    pub mode: OnetMode,
}

pub struct Onet {
    key: Key,
    config: OnetConfig,
    // dht: Dht,
    // hg: HgNode,
    // peers: Peers,
    // hg: Arc<Mutex<Hashgraph>>
}

impl Onet {
    pub fn new(config: OnetConfig) -> Onet {
        // let dht_config = DhtConfig {
        //     verbose: config.verbose,
        //     listen_addr: config.listen_addr,
        //     connect_addr: config.connect_addr,
        // };

        let hg_listen = config.listen_addr.clone();

        // hg_listen.set_port(config.listen_addr.port());

        let hg_config = NodeConfig {
            verbose: config.verbose,
            listen_addr: hg_listen,
            connect_addr: config.connect_addr,
        };

        // let dht = Dht::new(dht_config);
        //

        let key = Key::new_generate().unwrap();

        // let hg_key = HgKey {
        //     bytes: dht.key.clone().bytes,
        //     key_pair: dht.key.clone().key_pair,
        // };

        Onet {
            key: key.clone(),
            config,
            // dht: dht,
            // hg: HgNode::new(key, hg_config),
        }
    }

    pub fn run(&mut self) {
        // Dont bootstrap
        // self.dht.run(false);

        // Rpc::Duplex::add_plugin(ConnectWrapper {
        //     hash: self.dht.hash.clone(),
        //     pub_key: self.dht.key.get_pub(),
        //     hg: self.hg.clone(),
        // });

        // self.dht.bootstrap().unwrap();

        // JOIN
        //     connect_to_dht bootstrap node
        //     generate identity
        //     get closest node to pubk
        //     send joinRequest
        //

        match self.config.mode {
            OnetMode::Client => self.run_client(),
            OnetMode::Vault => self.run_vault(),
        }
    }

    pub fn run_client(&mut self) {
        // let mut client =
        //     ClientManagerRpc::connect_tcp(&self.config.connect_addr.unwrap().to_string()).unwrap();

        // let res = client.store_data("TAMERE".as_bytes().to_vec());

        // println!("RES {:#?}", res);

        // let res = client.get_data(res.unwrap().unwrap().unwrap());

        // println!("RES {:#?}", res);

        // let mut client2 =
        //     SectionRpc::connect_tcp(&self.config.connect_addr.unwrap().to_string()).unwrap();

        // let res = client2.store_data("TAMERE2".as_bytes().to_vec());
    }

    pub fn run_vault(&mut self) {
        // let mut section = Section::new(self.config.clone());

        // section.run();
        let identity = Identity::load(&self.config);
        let mut socket = Network::<TcpTransport>::new_default(&self.config.listen_addr);
        let mut vault = Vault::new(identity, &self.config, socket);

        if self.config.connect_addr.is_none() {
            vault.bootstrap();
        } else {
            vault.connect_bootstrap();
        }
    }
}

// #[derive(Clone)]
// struct ConnectWrapper {
//     hash: String,
//     pub_key: Vec<u8>,
//     hg: HgNode,
// }

// impl Wrapper for ConnectWrapper {
//     fn on_recv(&self, pack: &Packet) -> Packet {
//         let mut d = pack.data.clone();

//         // extract the hash
//         let mut pub_key = d.split_off(self.hash.len());

//         // extract the pubKey
//         pub_key.split_off(self.pub_key.len());

//         let mut sender = pack.header.sender.clone();

//         sender.set_port(sender.port() + 10);

//         self.hg
//             .peers
//             .write()
//             .unwrap()
//             .add(Peer::new(sender, pub_key.clone()));

//         pack.clone()
//     }
// }

// impl Debug for ConnectWrapper {
//     fn fmt(&self, fmt: &mut Formatter) -> FResult {
//         write!(fmt, "ConnectWrapper")
//     }
// }
