 rust
use std::net::{IpAddr, ToSocketAddrs};

struct Foo;

impl ToSocketAddrs for (IpAddr, Foo) {
    fn to_socket_addrs(&self) -> Result<ToSocketAddrs::Iter> {
        unimplemented!()
    }
}

fn main() {}
