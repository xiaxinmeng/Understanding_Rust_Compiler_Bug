rust
const fn decode_bytes(b: &[u8]) -> Option<u8> {
    match b {
        b"0" => Some(0),
        _ => None,
    }
}
