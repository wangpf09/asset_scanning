pub mod icmp;

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_parse_cidr() {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        icmp::parse_cidr(String::from("192.168.0.1/24"))
    }
}
