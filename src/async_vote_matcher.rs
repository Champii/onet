use crate::section::Hash;
use crate::vote_result::VoteResult;
use std::collections::HashMap;

pub struct AsyncVoteMatcher {
    waiting: HashMap<Hash, VoteResult>,
}

impl AsyncVoteMatcher {
    pub fn new() -> Self {
        Self {
            waiting: HashMap::new(),
        }
    }
}
