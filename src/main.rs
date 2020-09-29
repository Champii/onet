extern crate clap;
extern crate hashgraph;
extern crate logger;

#[macro_use]
extern crate rsrpc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
// extern crate rust_dht;
#[macro_use]
extern crate serde_derive;

mod args;
mod async_response_matcher;
mod async_vote_matcher;
mod datachain;
mod identity;
mod network_event;
mod onet;
mod routing;
mod rpc;
mod section;
mod storage;
mod vault;
mod vote_result;

use self::onet::Onet;

fn main() {
    let config = args::parse_config();

    logger::init_logger(config.verbose);

    let mut onet = Onet::new(config);

    onet.run();
}
