rust
pub fn try_from_matched(value: [u8; 4]) -> Result<Self, ()> {
    const ABCD: u32 = u32::from_ne_bytes(*b"ABCD");
    const EFGH: u32 = u32::from_ne_bytes(*b"EFGH");
    const IJKL: u32 = u32::from_ne_bytes(*b"IJKL");
    const MNOP: u32 = u32::from_ne_bytes(*b"MNOP");

    match u32::from_ne_bytes(value) {
        ABCD => Ok(Self::A),
        EFGH => Ok(Self::B),
        IJKL => Ok(Self::C),
        MNOP => Ok(Self::D),
        _ => Err(()),
    }
}
