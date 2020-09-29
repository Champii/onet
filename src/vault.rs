use crate::identity::Identity;
use crate::onet::OnetConfig;
use crate::routing::Routing;
use crate::rpc::RoutingRpc;
use crate::section::Section;
use crate::storage::Storage;
use rsrpc::Network;
use rsrpc::TcpTransport;

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

    pub fn bootstrap(&mut self) {
        info!("Bootstrap node");

        let mut section = Section::new(
            self.config.clone(),
            Routing::new(self.identity.cur_ident.clone()),
        );

        section.routing.write().unwrap().add(self.identity.clone());

        let mut net = Network::<TcpTransport>::new_default(&self.config.listen_addr);

        net.listen();

        section.run(net);

        self.section = Some(section);
    }

    pub fn connect_bootstrap(&mut self) {
        info!(
            "Connect to bootstrap node {}",
            self.config.connect_addr.unwrap().to_string()
        );

        let mut routing_client =
            RoutingRpc::connect_tcp(&self.config.connect_addr.clone().unwrap().to_string())
                .unwrap();

        let response = routing_client.bootstrap_vault(self.identity.clone());

        if let Ok(response) = response.unwrap() {
            info!("Joined section {} with {} vaults", 0, response.len());
            // If response OK
            // create section and connect to hg with section routing
            let mut section = Section::new(
                self.config.clone(),
                Routing::from(self.identity.cur_ident.clone(), response),
            );

            let mut net = Network::<TcpTransport>::new_default(&self.config.listen_addr);

            net.listen();

            section.run(net);

            self.section = Some(section);
        } else {
            error!("Bootstrap node and/or section refused this vault to join");
        }
    }
}
