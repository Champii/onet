extern crate clap;
extern crate hashgraph;
extern crate logger;
// extern crate log;
// extern crate rust_dht;

mod args;
mod onet;

use self::onet::Onet;

fn main() {
    let config = args::parse_config();

    logger::init_logger(config.verbose);

    let mut onet = Onet::new(config);

    onet.run();
}
