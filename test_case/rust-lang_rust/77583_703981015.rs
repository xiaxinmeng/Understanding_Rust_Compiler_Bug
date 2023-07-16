rust
fn main() {
    let ip: Ipv4Addr = std::env::args().nth(1).unwrap().parse().unwrap();
    let ipbeu32 = u32::from_be_bytes(ip.octets());
    let net = Ipv4Addr::from((ipbeu32 & 0xffffff00u32.to_be()).to_ne_bytes());
    println!("{}", net);
}
