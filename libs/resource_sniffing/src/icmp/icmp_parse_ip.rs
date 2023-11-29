use ipnet::Ipv4Net;
use log::{debug, error};

pub fn parse_cidr_ipv4(cidr: String) -> Vec<String> {
    debug!("解析cidr ip地址 {}", cidr);
    let mut ips: Vec<String> = Vec::new();
    let ip_range: Ipv4Net = cidr.parse::<Ipv4Net>().expect("无效的ip地址范围");
    for x in ip_range.hosts() {
        ips.push(x.to_string());
    }
    debug!("cidr 可用ip共: {}", ips.len());
    return ips;
}

pub fn parse_ipv4_range(ip_range: String) -> Vec<String> {
    debug!("解析ip地址范围 {}", ip_range);
    let mut ips: Vec<String> = Vec::new();
    let temp_ip: Vec<String> = split2vec_str(&ip_range, '-');
    if temp_ip.len() != 2 {
        error!("IP地址范围格式错误: {}", ip_range);
        return ips;
    }
    let end_ip = &temp_ip[1];
    if end_ip.len() < 4 {
        let x = split2vec_int(&temp_ip[0], '.');
        if x.len() != 4 {
            error!("IP地址范围格式错误: {}", ip_range);
            return ips;
        }
        let end = format!("{}.{}.{}.{}", x[0], x[1], x[2], end_ip);
        ips.extend(calc_ipv4_range(&temp_ip[0], &end));
    } else {
        ips.extend(calc_ipv4_range(&temp_ip[0], &temp_ip[1]));
    }
    debug!("ip range 可用ip共: {}", ips.len());
    return ips;
}

fn calc_ipv4_range(ip1: &String, ip2: &String) -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();
    let start_ip = split2vec_int(&ip1, '.');
    let end_ip = split2vec_int(&ip2, '.');
    if start_ip.len() != 4 || end_ip.len() != 4 {
        error!("IP地址范围格式错误: {}-{}", ip1, ip2);
        return ips;
    }
    let start = start_ip[0] << 24 | start_ip[1] << 16 | start_ip[2] << 8 | start_ip[3];
    let end = end_ip[0] << 24 | end_ip[1] << 16 | end_ip[2] << 8 | end_ip[3];
    for i in start..end {
        let ip = format!("{}.{}.{}.{}", (i >> 24) & 0xff, (i >> 16) & 0xff, (i >> 8) & 0xff, i & 0xff);
        ips.push(ip);
    }
    return ips;
}

fn split2vec_str(string: &String, delimiter: char) -> Vec<String> {
    string.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

fn split2vec_int(string: &String, delimiter: char) -> Vec<i32> {
    string.split(delimiter)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
