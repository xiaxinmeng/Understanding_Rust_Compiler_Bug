
> use std::net::Ipv4Addr;
>
> let addr = Ipv4Addr::new(13, 12, 11, 10);
> assert_eq!(0x0d0c0b0au32, u32::from(addr));
> 