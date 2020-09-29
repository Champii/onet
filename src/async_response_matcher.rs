use crate::section::Hash;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct AsyncResponseMatcher {
    waiting: HashMap<Hash, Sender<Vec<u8>>>,
}

impl AsyncResponseMatcher {
    pub fn new() -> Self {
        Self {
            waiting: HashMap::new(),
        }
    }

    pub fn add(&mut self, hash: Hash, tx: Sender<Vec<u8>>) {
        trace!("Add waiting {:?}", hash);

        self.waiting.insert(hash, tx);
    }

    pub fn resolve(matcher: &mut AsyncResponseMatcher, hash: Hash, data: Vec<u8>) {
        trace!("Resolve waiting {:?}", hash);

        match matcher.waiting.remove(&hash) {
            Some(tx) => tx.send(data).unwrap(),
            None => trace!("Cannot find such answer ! {:?}", hash),
        };
    }

    pub fn remove(matcher: &mut AsyncResponseMatcher, hash: Hash) {
        trace!("Remove waiting {:?}", hash);

        matcher.waiting.remove(&hash).unwrap();
    }

    pub fn close(&mut self) {
        for (_, tx) in &self.waiting {
            drop(tx);
        }
    }
}
