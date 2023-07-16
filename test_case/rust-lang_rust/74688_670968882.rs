rust
pub fn problematic(buf: &[u8]) -> &[u8] {
    for i in 0 .. buf.len() {
        if buf[i] == 0x00 {
            return &buf[i..];
        }
    }

    &[]
}
