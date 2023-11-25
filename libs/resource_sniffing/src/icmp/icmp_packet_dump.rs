use std::net::{IpAddr, Ipv4Addr};

use log::debug;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::icmp::{echo_request, IcmpPacket, IcmpTypes};
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::transport;
use pnet::transport::{icmp_packet_iter, TransportChannelType, TransportProtocol};

const BUFFER_SIZE: usize = 4096;
const IPV4_HEADER_LEN: usize = 20;
const ICMP_HEADER_LEN: usize = 8;

pub fn handle_icmp_ipv4_packet(target_ip: String) -> Result<(), String> {
    let protocol = TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp);
    let (mut tx, mut rx) = transport::transport_channel(
        BUFFER_SIZE, TransportChannelType::Layer4(protocol),
    ).map_err(|e| format!("Error create transport channel: {}", e))?;

    let mut icmp_header = vec![0u8; IPV4_HEADER_LEN + ICMP_HEADER_LEN];
    let icmp_packet = build_icmp_echo_request(&mut icmp_header);
    let x: IpAddr = target_ip.parse().unwrap();

    tx.send_to(icmp_packet, x)
        .map_err(|e| format!("Error sending ICMP echo request: {}", e))?;

    let mut iter = icmp_packet_iter(&mut rx);
    match iter.next() {
        Ok((packet, addr)) => match EchoReplyPacket::new(packet.packet()) {
            Some(echo_reply) => {
                debug!("addr: {:?} icmp: {:?}", addr, echo_reply.get_icmp_type());
                Ok(())
            }
            None => Err("received none".to_string())
        }
        Err(e) => Err(e.to_string())
    }
}

fn build_icmp_echo_request<'a>(icmp_header: &'a mut [u8]) -> MutableIpv4Packet<'a> {
    let mut ipv4_packet = MutableIpv4Packet::new(icmp_header).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_total_length((IPV4_HEADER_LEN + ICMP_HEADER_LEN) as u16);
    ipv4_packet.set_source(Ipv4Addr::new(0, 0, 0, 0));
    ipv4_packet.set_destination(Ipv4Addr::new(127, 0, 0, 1));
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);

    let mut icmp_packet = echo_request::MutableEchoRequestPacket::new(ipv4_packet.payload_mut()).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_identifier(1);
    icmp_packet.set_sequence_number(1);

    ipv4_packet
}

fn is_icmp_echo_reply(icmp_reply: &[u8]) -> bool {
    let icmp_packet = IcmpPacket::new(icmp_reply).unwrap();
    icmp_packet.get_icmp_type() == IcmpTypes::EchoReply
}