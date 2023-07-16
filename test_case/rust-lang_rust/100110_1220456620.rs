rust
use std::net::ToSocketAddrs;

pub fn main() {
    println!("before");
    let _ = "localhost:8080".to_socket_addrs(); // will segfault
    std::net::TcpStream::connect("localhost:8080").unwrap(); // will also segfault
    println!("hello world");
}
