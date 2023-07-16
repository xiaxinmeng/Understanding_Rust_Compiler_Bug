rust
trait ByteConversions {
    const N: usize;
    pub fn to_bytes(self) -> [u8; Self::N];
    pub fn from_bytes(bytes: [u8; Self::N]) -> Self;
}
