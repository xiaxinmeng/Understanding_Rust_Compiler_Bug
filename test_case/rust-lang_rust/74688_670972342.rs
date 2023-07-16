rust
pub fn problematic(buf: &[u8]) -> &[u8] {
    for (i, b) in buf.iter().enumerate() {
        // this was a matcher, simplified it for this
        if *b == 0x00 {
            return &buf[i..];
        }
    }

    &[]
}
