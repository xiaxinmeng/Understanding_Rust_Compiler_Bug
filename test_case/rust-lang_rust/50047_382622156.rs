rust
pub fn bar(buf: [u8; 4096]) -> Box<[u8; 4096]> {
    Box::new(buf)
}
