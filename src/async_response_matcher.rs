use crate::section::Hash;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct AsyncResponseMatcher<T> {
    waiting: HashMap<Hash, Sender<T>>,
}

impl<T> AsyncResponseMatcher<T> {
    pub fn new() -> Self {
        Self {
            waiting: HashMap::new(),
        }
    }

    pub fn add(&mut self, hash: Hash, tx: Sender<T>) {
        trace!("Add waiting {:?}", hash);

        self.waiting.insert(hash, tx);
    }

    pub fn resolve(matcher: &mut AsyncResponseMatcher<T>, hash: Hash, data: T) {
        trace!("Resolve waiting {:?}", hash);

        match matcher.waiting.remove(&hash) {
            Some(tx) => tx.send(data).unwrap(),
            None => trace!("Cannot find such answer ! {:?}", hash),
        };
    }

    pub fn remove(matcher: &mut AsyncResponseMatcher<T>, hash: Hash) {
        trace!("Remove waiting {:?}", hash);

        matcher.waiting.remove(&hash).unwrap();
    }

    pub fn close(&mut self) {
        for (_, tx) in &self.waiting {
            drop(tx);
        }
    }
}
