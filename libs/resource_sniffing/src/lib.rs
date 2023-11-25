pub mod icmp;

#[cfg(test)]
mod tests {
    use std::env;

    use log::debug;

    use super::*;

    #[test]
    fn test_parse_cidr() {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let cidrs = vec![
            String::from("192.168.124.1/24")
        ];
        let ips = icmp::parse_cidr_ipv4(cidrs);
        for ip in ips {
            debug!("ip {}", ip)
        }
    }

    #[test]
    fn test_check_alive_ipv4() {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        icmp::check_alive_ipv4()
    }
}
