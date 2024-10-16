use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use log::{debug, info, warn};
use rayon::prelude::*;

mod icmp_parse_ip;

pub fn parse_cidr_ipv4(cidrs: Vec<String>) -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();
    for ci in cidrs {
        if ci.contains("/") {
            ips.extend(icmp_parse_ip::parse_cidr_ipv4(ci));
        } else if ci.contains("-") {
            ips.extend(icmp_parse_ip::parse_ipv4_range(ci));
        } else if ci.parse::<Ipv4Addr>().is_ok() {
            ips.push(ci)
        } else {
            warn!("当前cidr: {} 不满足ipv4规则，将被舍弃!!!", ci)
        }
    }
    info!("ip地址全部解析完成，可用ip共 {}", ips.len());
    ips
}

fn check_ip_reachable(ip: String) -> bool {
    let timeout = Duration::from_secs(1);

    match ping::ping(ip.parse().unwrap(),
                     Some(timeout), Some(166), Some(3),
                     Some(5), Some(&rand::random())) {
        Ok(_) => {
            true
        }
        Err(e) => {
            debug!("{:?} {} is unreachable. {}",thread::current().id(), ip, e);
            false
        }
    }
}

pub fn reachable_ips(cidrs: Vec<String>) -> Vec<String> {
    let all_ips = parse_cidr_ipv4(cidrs);
    let ips = Arc::new(Mutex::new(Vec::new()));

    all_ips.par_iter().for_each(|ip| {
        let mut guard = ips.lock().unwrap();
        if check_ip_reachable(ip.clone()) {
            guard.push(ip.clone())
        }
    });
    let ips = Arc::try_unwrap(ips).unwrap().into_inner().unwrap();
    ips
}

