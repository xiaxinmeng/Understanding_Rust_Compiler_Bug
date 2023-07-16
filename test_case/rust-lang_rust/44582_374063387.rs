rust
        Ipv4Addr {
            inner: c::in_addr {
                // 127.0.0.1
                #[cfg(target_endian = "big")]
                s_addr: 0x7F_00_00_01_u32,
                #[cfg(target_endian = "little")]
                s_addr: 0x01_00_00_7F_u32,
            }
        }
