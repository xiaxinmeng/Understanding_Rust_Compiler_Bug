rust
/// fn string_from_utf8_lossy(input: &[u8]) -> String {
///     let mut string = String::new();
///     let mut decoder = utf8::Decoder::new();
///     for piece in decoder.feed(input) {
///         string.push_str(&piece)
///     }
///     if let Some(piece) = decoder.end() {
///         string.push_str(&piece)
///     }
///     string
/// }
/// 