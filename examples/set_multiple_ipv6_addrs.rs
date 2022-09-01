use std::net::Ipv6Addr;
use std::str::FromStr;

use tidy_tuntap::*;

fn main() {
    let tun = tun::Tun::without_packet_info("tun10").unwrap();

    tun.set_ipv6_addr(Ipv6Addr::from_str("fe80::be8f:5838:c7ca:b98").unwrap())
        .unwrap();
    tun.set_ipv6_addr(Ipv6Addr::from_str("fe80::be8f:5838:c7ca:b99").unwrap())
        .unwrap();

    let ipv6_addrs = tun.get_ipv6_addrs().unwrap();

    assert_eq!(ipv6_addrs.len(), 2);
    assert!(ipv6_addrs.contains(&Ipv6Addr::from_str("fe80::be8f:5838:c7ca:b98").unwrap()));
    assert!(ipv6_addrs.contains(&Ipv6Addr::from_str("fe80::be8f:5838:c7ca:b99").unwrap()));
}
