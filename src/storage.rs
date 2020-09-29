use bincode::serialize;
use hex::encode;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

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
