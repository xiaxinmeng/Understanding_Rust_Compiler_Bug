rust
pub fn char_decode_from_utf8(bytes: &[u8]) -> Option<char> {
    let decoded = std::str::from_utf8(bytes).ok()?;
    let mut chars = decoded.chars();
    let result = chars.next()?;
    match chars.next() {
        None => Some(result),
        Some(_) => None, // `bytes` contains more than 1 codepoint!
    }
}

