use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use log::{debug, info, warn};

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
    return ips;
}

pub fn check_ip_reachable(ip: &String) -> bool {
    let timeout = Duration::from_secs(1);
    let reached = ping::ping(ip.parse().unwrap(),
                             Some(timeout), Some(166), Some(3),
                             Some(5), Some(&rand::random()));

    match reached {
        Ok(_) => true,
        Err(e) => {
            let thread_id = thread::current().id();
            debug!("cur thread: {:?} IP: {} is unreachable. err: {}",thread_id, ip, e);
            return false;
        }
    }
}

fn check_ip_reachable_with_thread(ips: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for ip in ips {
        if check_ip_reachable(&ip) {
            res.push(ip)
        }
    }
    res
}


pub fn reachable_ips(cidrs: Vec<String>, thread_num: usize) -> Vec<String> {
    let chunkes = chunk_list(parse_cidr_ipv4(cidrs), thread_num);
    let ips = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for chunk in chunkes {
        let ips_clone = Arc::clone(&ips);
        let handle = thread::spawn(move || {
            let mut guard = ips_clone.lock().unwrap();
            let res = check_ip_reachable_with_thread(chunk);
            guard.extend(res);
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap()
    }
    let ips = Arc::try_unwrap(ips).unwrap().into_inner().unwrap();
    ips
}

fn chunk_list<T>(data: Vec<T>, chunk_num: usize) -> Vec<Vec<T>> where T: Clone, {
    let len = data.len();
    let size = (len + chunk_num - 1) / chunk_num;
    data.chunks(size).map(|chunk| chunk.to_vec()).collect()
}