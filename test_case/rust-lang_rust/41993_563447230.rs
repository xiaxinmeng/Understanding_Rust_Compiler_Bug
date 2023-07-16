rust
pub fn checkchar(a: &str) -> bool {
    let mut buf = [0; 4];
    a.starts_with(&*'/'.encode_utf8(&mut buf[..]))
}
