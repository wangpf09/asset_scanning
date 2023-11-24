use std::net::Ipv4Addr;

use log::warn;

mod icmp_sniffing;


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
    return ips;
}
