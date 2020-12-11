use crate::identity::Identity;
use crate::routing::Routing;
use crate::rpc::ServiceClient;
use crate::section::Hash;
use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use std::sync::RwLock;
use tarpc::client;
use tarpc::context;
use tokio_serde::formats::Bincode;
use crate::section::HgNode;

#[derive(Clone)]
pub struct RoutingService {
    pub routing: Arc<RwLock<Routing>>,
    pub hg: Arc<RwLock<HgNode>>,
}


impl RoutingService {
    pub async fn bootstrap_vault(&self, identity: Identity) -> Result<HashMap<Hash, Identity>, String> {
        debug!("Bootstrap Vault");
        // generate new RuntimeIdentity : hash(ownID + pub_key)
        let addr = {
            let routing = self.routing.read().unwrap();

            let datamanager = routing.get_nearest_of(identity.cur_ident.clone()).unwrap();

            datamanager.listening_addr.clone()
        };

        let mut client = crate::rpc::client(addr).await;

        let response = client
            .ask_join(context::current(), identity.clone())
            .await.unwrap();

        match response {
            Ok(accepted) => {
                if accepted {
                    let mut routing = self.routing.write().unwrap();

                    routing.add(identity);

                    // send ok + section routing
                    Ok(routing.section_members.clone())
                } else {
                    Err("Not accepted".to_string())
                }
            }
            Err(err) => Err(err),
        }
    }
}
