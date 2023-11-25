use std::net::Ipv4Addr;

use log::{error, info, warn};

mod icmp_sniffing;
mod icmp_packet_dump;

pub fn check_alive_ipv4() {
    let result = icmp_packet_dump::handle_icmp_ipv4_packet("127.0.0.1".to_string());
    match result {
        Ok(_) => info!("host is reachable"),
        Err(e) => error!("host is not reachable: {}", e)
    }
}

pub fn parse_cidr_ipv4(cidrs: Vec<String>) -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();
    for ci in cidrs {
        if ci.contains("/") {
            ips.extend(icmp_sniffing::parse_cidr_ipv4(ci));
        } else if ci.contains("-") {
            ips.extend(icmp_sniffing::parse_ipv4_range(ci));
        } else if ci.parse::<Ipv4Addr>().is_ok() {
            ips.push(ci)
        } else {
            warn!("当前cidr: {} 不满足ipv4规则，将被舍弃!!!", ci)
        }
    }
    info!("ip地址全部解析完成，可用ip共 {}", ips.len());
    return ips;
}
