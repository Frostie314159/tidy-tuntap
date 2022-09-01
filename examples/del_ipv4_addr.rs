use std::net::Ipv4Addr;

use tidy_tuntap::*;

fn main() {
    let tun = tun::Tun::without_packet_info("tun10").unwrap();

    tun.set_addr(Ipv4Addr::new(10, 10, 10, 10)).unwrap();
    tun.del_addr().unwrap();

    let err = tun.get_addr().unwrap_err();

    assert!(matches!(
        err,
        error::Error::NixError(nix::errno::Errno::EADDRNOTAVAIL)
    ))
}
