
#![feature(ip_constructors)]
use std::net::Ipv4Addr;

fn test_ipv4_addr_portable_conversions() {
    assert_eq!(Ipv4Addr::localhost(), Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(Ipv4Addr::localhost(), Ipv4Addr::from(0x7F000001));
    assert_eq!(Ipv4Addr::localhost(), Ipv4Addr::from([127u8, 0u8, 0u8, 1u8]));
    assert_eq!(Ipv4Addr::localhost(), "127.0.0.1".parse::<Ipv4Addr>().unwrap());

    assert_eq!(u32::from(Ipv4Addr::localhost()), 0x7F000001);
    assert_eq!(Ipv4Addr::localhost().octets(), [127u8, 0u8, 0u8, 1u8]);
    assert_eq!(Ipv4Addr::localhost().to_string(), "127.0.0.1");
}

fn test_ipv4_network_endian_transmutate_to_bytes() {
    let host_byte_order_u32 : u32 = u32::from(Ipv4Addr::localhost());
    assert_eq!(host_byte_order_u32, 0x7F000001);
    let network_byte_order_u32: u32 = host_byte_order_u32.to_be();
    assert_eq!(network_byte_order_u32, if cfg!(target_endian = "big") { 0x7F000001 } else { 0x0100007F });
    let bytes : [u8; 4] = unsafe { std::mem::transmute(network_byte_order_u32) };
    assert_eq!(bytes, [127u8, 0u8, 0u8, 1u8]);
}

fn test_ipv4_network_endian_transmutate_from_bytes() {
    let bytes : [u8; 4] = [127u8, 0u8, 0u8, 1u8];
    let network_byte_order_u32 : u32 = unsafe { std::mem::transmute(bytes) };
    assert_eq!(network_byte_order_u32, if cfg!(target_endian = "big") { 0x7F000001 } else { 0x0100007F });
    let host_byte_order_u32 : u32 = u32::from_be(network_byte_order_u32);
    assert_eq!(0x7F000001, host_byte_order_u32);
    assert_eq!(Ipv4Addr::localhost(), Ipv4Addr::from(host_byte_order_u32));
}
