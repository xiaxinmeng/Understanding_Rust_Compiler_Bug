 rust
use std::net::Ipv6Addr;
use std::str::FromStr;

fn main() {
    let mut ips = vec![Ipv6Addr::from_str("2001:db8:f00::1002").unwrap(),
                       Ipv6Addr::from_str("2001:db8:f00::1004").unwrap(),
                       Ipv6Addr::from_str("2001:db8:f00::2001").unwrap(),
                       Ipv6Addr::from_str("2001:db8:f00::2003").unwrap(),
                       Ipv6Addr::from_str("2001:db8:baa::1002").unwrap(),
                       Ipv6Addr::from_str("2001:db8:baa::1004").unwrap(),
                       Ipv6Addr::from_str("2001:db8:baa::2001").unwrap(),
                       Ipv6Addr::from_str("2001:db8:baa::2003").unwrap(),];

    ips.sort();
    for ip in ips.iter() {
        println!("ip: {}", ip);
    }
}
