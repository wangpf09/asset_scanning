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
            String::from("192.168.0.1/24"),
            String::from("127.0.0.1-127.0.0.15"),
        ];
        let ips = icmp::parse_cidr(cidrs);
        for ip in ips {
            debug!("ip {}", ip)
        }
    }
}
