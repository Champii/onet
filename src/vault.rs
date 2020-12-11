use crate::rpc::ServiceClient;
use crate::identity::Identity;
use crate::onet::OnetConfig;
use crate::routing::Routing;
use crate::section::Section;
use crate::storage::Storage;
use rsrpc::Network;
use rsrpc::TcpTransport;
use tokio_serde::formats::Bincode;
use tarpc::client;
use tarpc::context;

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

    pub async fn bootstrap(&mut self) -> std::io::Result<()> {
        info!("Bootstrap node");

        let mut section = Section::new(
            self.config.clone(),
            Routing::new(self.identity.cur_ident.clone()),
        );

        section.routing.write().unwrap().add(self.identity.clone());

        // let mut net = Network::<TcpTransport>::new_default(&self.config.listen_addr);

        // net.listen();

        section.run();
        crate::rpc::listen(self.config.listen_addr.clone(), section.routing.clone(), section.hg.clone()).await;

        self.section = Some(section);

        Ok(())
    }

    pub async fn connect_bootstrap(&mut self) -> std::io::Result<()> {
        info!(
            "Connect to bootstrap node {}",
            self.config.connect_addr.unwrap().to_string()
        );

        let mut section = Section::new(
            self.config.clone(),
            Routing::new(self.identity.cur_ident.clone()),
        );

        let connect_addr = self.config.connect_addr.clone().unwrap();

        let mut client = crate::rpc::client(connect_addr.to_string()).await;

        let response = client
            .bootstrap_vault(context::current(), self.identity.clone())
            .await;

        println!("RESPONSE {:?}", response);
            let mut section = Section::new(
                self.config.clone(),
                Routing::from(self.identity.cur_ident.clone(), response.unwrap().unwrap()),
            );

        section.run();
        crate::rpc::listen(self.config.listen_addr.clone(), section.routing.clone(), section.hg.clone()).await;

        self.section = Some(section);

        Ok(())
    }
}
