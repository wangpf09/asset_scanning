use log::{debug, info};

use banner::Banner;
use command::Opts;
use resource_sniffing::icmp;

mod banner;
mod command;
mod logger;

fn main() {
    logger::init_logger();
    Banner::print();

    let opts = Opts::read();
    info!("cidr address {:?}", opts.address);
    info!("----------- 开始进行存活资产检测 -----------");
    let ips = icmp::reachable_ips(opts.address);
    info!("可达ip共有: {}", ips.len());
    for x in ips {
        debug!("{}", x)
    }
    info!("----------- 存活资产检测结束 -----------");
}
