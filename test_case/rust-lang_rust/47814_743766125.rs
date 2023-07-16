
struct ArpIPv4 {
    s: u8
}

impl ArpIPv4 {
    const LENGTH: usize = 20;

    pub fn to_buffer() -> [u8; Self::LENGTH] {
        unimplemented!()
    }
}
