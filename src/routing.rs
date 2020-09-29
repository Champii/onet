use crate::identity::Identity;
use crate::section::Hash;
use std::collections::HashMap;
use xor::xor;

#[derive(Debug, Clone)]
pub struct Routing {
    pub own_hash: Hash,
    pub section_members: HashMap<Hash, Identity>,
    pub sibling_elders: HashMap<Hash, Identity>, // section elders from sibling sections -> identity
}

impl Routing {
    pub fn new(own_hash: Hash) -> Self {
        Self {
            own_hash,
            section_members: HashMap::new(),
            sibling_elders: HashMap::new(),
        }
    }

    pub fn from(own_hash: Hash, section_members: HashMap<Hash, Identity>) -> Self {
        Self {
            own_hash,
            section_members: section_members,
            sibling_elders: HashMap::new(),
        }
    }

    pub fn add(&mut self, identity: Identity) {
        self.section_members
            .insert(identity.cur_ident.clone(), identity);
    }

    pub fn remove(&mut self, hash: Hash) {
        self.section_members.remove(&hash);
    }

    pub fn get_farthest(&mut self) -> Option<Identity> {
        let mut max = &self.own_hash;
        let mut identity = None;

        for (hash, identity_) in self.section_members.iter() {
            if self.is_farther(&hash, max) {
                max = hash;

                identity = Some(identity_.clone());
            }
        }

        identity
    }

    pub fn get_nearest_of(&self, h: Hash) -> Option<Identity> {
        let mut min = &self.own_hash;
        let mut identity = None;

        for (hash, identity_) in self.section_members.iter() {
            if self.is_nearer_of(&hash, &min, &h) {
                min = hash;

                identity = Some(identity_.clone());
            }
        }

        identity
    }

    pub fn is_farther(&self, h1: &Hash, h2: &Hash) -> bool {
        let dist1 = Self::xor_distance(&self.own_hash, h1);
        let dist2 = Self::xor_distance(&self.own_hash, h2);

        if let Greater = dist1.cmp(&dist2) {
            true
        } else {
            false
        }
    }

    #[allow(unused)]
    pub fn is_nearer(&self, h1: &Hash, h2: &Hash) -> bool {
        let dist1 = Self::xor_distance(&self.own_hash, h1);
        let dist2 = Self::xor_distance(&self.own_hash, h2);

        if let Less = dist1.cmp(&dist2) {
            true
        } else {
            false
        }
    }

    pub fn is_nearer_of(&self, h1: &Hash, h2: &Hash, of: &Hash) -> bool {
        let dist1 = Self::xor_distance(of, h1);
        let dist2 = Self::xor_distance(of, h2);

        if let Less = dist1.cmp(&dist2) {
            true
        } else {
            false
        }
    }

    pub fn xor_distance(s1: &Hash, s2: &Hash) -> Hash {
        let a1 = &s1;
        let a2 = &s2;

        let result = xor(a1, a2);

        return result;

        // if let Ok(res) = String::from_utf8(result) {
        //     return hex::encode(res);
        // }

        // String::from("")
    }
}
