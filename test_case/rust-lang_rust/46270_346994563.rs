rust
use std::net::*;
fn main() {
    let a = IpAddr::V6(Ipv6Addr::new(1,2,3,4,5,6,7,8));
    let b = SocketAddr::new(a, 9);
    println!("{:?}", a);
    println!("{:?}", b);
    assert_eq!(b.ip(), a);
}
