rust
use std::net::{SocketAddr, ToSocketAddrs};

fn main() {
    // Convert input to `Debug` fmt'able form.
    fn tsa<A: ToSocketAddrs>(a: A) -> Result<Vec<SocketAddr>, String> {
        match a.to_socket_addrs() {
            Ok(a) => Ok(a.collect()),
            Err(e) => Err(e.to_string()),
        }
    }

    println!("{:?}", tsa("1200::AB00:1234::2552:7777:1313:34300"));
}
