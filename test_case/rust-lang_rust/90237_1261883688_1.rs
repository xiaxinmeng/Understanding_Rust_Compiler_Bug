rust
const fn decode_string(s: &str) -> Option<u8> {
    match s {
        "0" => Some(0),
        _ => None,
    }
}
