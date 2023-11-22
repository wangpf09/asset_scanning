use ipnet::Ipv4Net;
use log::debug;

mod icmp_sniffing;


pub fn parse_cidr(cidr: String) {
    let ip_range: Ipv4Net = cidr.parse::<Ipv4Net>().expect("无效的ip地址范围");
    for x in ip_range.hosts() {
        debug!("{}", x)
    }
    icmp_sniffing::parse_ip()
}
