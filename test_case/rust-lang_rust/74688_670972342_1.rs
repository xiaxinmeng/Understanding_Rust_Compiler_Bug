rust
pub fn solution(mut buf: &[u8]) -> &[u8] {
    while !buf.is_empty() {
        // just simplifying this here, in the final code it's a match checking more things
        if buf[0] == 0x00 {
            break;
        } else {
            buf = &buf[1..];
        }
    }

    buf
}
