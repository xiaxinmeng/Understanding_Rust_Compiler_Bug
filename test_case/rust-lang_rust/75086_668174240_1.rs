rust
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        // s_addr is stored in big-endian on all machines, so the array is provided in big-endian and
        // the native endian conversion method is used so that it's never swapped.
        Ipv4Addr { inner: c::in_addr { s_addr: u32::from_ne_bytes([a, b, c, d]) } }
    }
