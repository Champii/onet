use crate::section::Hash;
use crate::vote_result::VoteResult;
use std::collections::HashMap;

#[derive(Default)]
pub struct AsyncVoteMatcher {
    pub waiting: HashMap<Hash, VoteResult>,
}

impl AsyncVoteMatcher {
    pub fn new() -> Self {
        Self {
            waiting: HashMap::new(),
        }
    }
}
