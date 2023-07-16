 rust
use std::net::{SocketAddr, SocketAddrV6, UdpSocket};

fn main() {
    let socket = UdpSocket::bind(&SocketAddr::V6(SocketAddrV6::new("fe80::1".parse().unwrap(), 0xfc00, 0, 2))).unwrap();
}
