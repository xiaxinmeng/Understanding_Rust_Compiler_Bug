rust
fn append(buf: &mut &mut [u8]) {
    *buf = &mut buf[..];
}
