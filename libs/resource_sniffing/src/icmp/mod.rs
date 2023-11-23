mod icmp_sniffing;


pub fn parse_cidr(cidrs: Vec<String>) {
    for ci in cidrs {
        if ci.contains("/") {
            icmp_sniffing::parse_cidr_ipv4(ci);
        } else if ci.contains("-") {
            icmp_sniffing::parse_ipv4_range(ci);
        }
    }
}
