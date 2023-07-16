rust
use rand::Rng;
use std::net::Ipv4Addr;

fn main() {
    let mut rng = rand::thread_rng();
    let ips: Vec<_> = (0..100_000)
        .map(|_| Ipv4Addr::from(rng.gen::<u32>()))
        .collect();
}
