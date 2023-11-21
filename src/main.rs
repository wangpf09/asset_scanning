use std::env;

use ipnet::Ipv4Net;
use log::debug;

use banner::Banner;
use command::Opts;

mod banner;
mod command;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    Banner::print();

    let opts = Opts::read();
    debug!("opts address {}", opts.config);

    let ip_range: Ipv4Net = "192.168.0.1/10".parse::<Ipv4Net>().expect("无效的ip地址范围");
    for x in ip_range.hosts() {
        debug!("{}", x)
    }
}
