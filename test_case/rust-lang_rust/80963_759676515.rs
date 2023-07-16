rust
pub fn foo(buf: &[u8]) -> Option<&[u8]> {
    if buf.len() > usize::MAX/3 {
        return None;
    }
    let n = (3*buf.len())/4;
    let n = std::cmp::min(n, buf.len());
    Some(&buf[..n])
}
