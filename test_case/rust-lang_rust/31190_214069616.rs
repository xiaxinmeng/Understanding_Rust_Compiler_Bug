 rust
fn from_char_buf(buf: &[c_char]) -> &[u8] {
    let len = buf.iter()
        .enumerate()
        .find(|&(_, &byte)| byte == 0)
        .map_or_else(|| buf.len(), |(len, _)| len);
    &buf[..len]
}
