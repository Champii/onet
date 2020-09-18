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
mod onet;
mod rpc;
mod section;

use self::onet::Onet;

fn main() {
    let config = args::parse_config();

    logger::init_logger(config.verbose);

    let mut onet = Onet::new(config);

    onet.run();
}
