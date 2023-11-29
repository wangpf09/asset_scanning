use std::env;

use log::{debug, info};

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
    let ips = icmp::reachable_ips(opts.address, 5);
    info!("可达ip共有: {}", ips.len());
    for x in ips {
        debug!("{}", x)
    }
}
