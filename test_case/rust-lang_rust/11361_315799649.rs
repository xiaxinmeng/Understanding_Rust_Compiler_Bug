
fn append(buf: &mut &[u8]) {
    *buf = &buf[..];
}
