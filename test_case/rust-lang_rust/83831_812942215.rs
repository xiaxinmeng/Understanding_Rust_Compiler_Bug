rust
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn ipv4_bitand_mask(ipv4: Ipv4Addr, mask: u32) -> Ipv4Addr {
    let ipv4_u32 = u32::from_ne_bytes(ipv4.octets());
    Ipv4Addr::from((ipv4_u32 & mask).to_ne_bytes())
}

pub fn ipv6_bitand_mask(ipv6: Ipv6Addr, mask: u128) -> Ipv6Addr {
    let ipv6_u128 = u128::from_ne_bytes(ipv6.octets());
    Ipv6Addr::from((ipv6_u128 & mask).to_ne_bytes())
}
