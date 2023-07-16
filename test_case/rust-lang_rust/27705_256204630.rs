 rust
use std::net::ToSocketAddrs;
use std::io;

fn resolve(host: &str) -> io::Result<Vec<IpAddr>> {
    (s, 0).to_socket_addrs().map(|iter| iter.map(|socket_address| socket_address.ip()).collect())
}
