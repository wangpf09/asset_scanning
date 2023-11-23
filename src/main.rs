use std::env;

use log::debug;

use banner::Banner;
use command::Opts;
use resource_sniffing::icmp;

mod banner;
mod command;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    Banner::print();

    let opts = Opts::read();
    debug!("opts address {:?}", opts.address);

    icmp::parse_cidr(opts.address)
}
