pub mod icmp;

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_parse_cidr() {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let cidrs = vec![
            String::from("192.168.0.1/24"),
            String::from("127.0.0.1-127.0.0.15")
        ];
        icmp::parse_cidr(cidrs)
    }
}
