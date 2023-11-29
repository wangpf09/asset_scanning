pub mod icmp;

#[cfg(test)]
mod tests {
    use std::env;

    use log::{debug, warn};

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
        if icmp::check_ip_reachable(&"127.0.0.1".to_string()) {
            debug!("ip reach");
            return;
        }
        warn!("ip un reach")
    }
}
