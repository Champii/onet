use crate::onet::OnetConfig;
use crate::section::Hash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identity {
    pub pub_key: Vec<u8>, // Key ?
    pub cur_ident: Hash,  // Key ?
    pub listening_addr: String,
}

impl Identity {
    pub fn load(config: &OnetConfig) -> Self {
        // // TODO: put that in config
        // let path = PathBuf::from("~/.onet/identity");

        // let res = std::fs::read(path.clone())
        //     .or(Err("Error".to_string()))
        //     .unwrap();

        // bincode::deserialize(&res).unwrap()
        let mut key: Vec<u8> = Vec::new();

        for _ in 0..20 {
            key.push(rand::random::<u8>());
        }

        Self {
            pub_key: key.clone(),
            cur_ident: key.clone(),
            listening_addr: config.listen_addr.to_string(),
        }
    }
}
