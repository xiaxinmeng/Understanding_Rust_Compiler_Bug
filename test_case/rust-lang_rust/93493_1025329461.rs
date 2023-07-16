rust
pub fn demo(c: char) -> bool {
    match c {
        '\0' ..= '\u{D7FF}' => false,
        '\u{E000}' ..= '\u{10FFFF}' => true,
    }
}
