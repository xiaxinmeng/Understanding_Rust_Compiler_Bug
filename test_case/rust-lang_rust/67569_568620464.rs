rust
pub fn try_encode_utf8(x: char, buf: &mut [u8]) -> Option<&mut str> {
    if x.len_utf8() <= buf.len() {
        Some(x.encode_utf8(&mut buf[..]))
    } else {
        None
    }
}

pub fn encode_utf8_4(x: char, buf: &mut [u8; 4]) -> &mut str {
    x.encode_utf8(&mut buf[..])
}
