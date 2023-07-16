 rust
unsafe { &*(self as *const PacketHeader as *const [u8; HEADER_SIZE]) }
