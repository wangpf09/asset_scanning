mod icmp_sniffing;


pub fn parse_cidr(cidrs: Vec<String>) -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();
    for ci in cidrs {
        if ci.contains("/") {
            ips.extend(icmp_sniffing::parse_cidr_ipv4(ci));
        } else if ci.contains("-") {
            ips.extend(icmp_sniffing::parse_ipv4_range(ci));
        }
    }
    return ips;
}
