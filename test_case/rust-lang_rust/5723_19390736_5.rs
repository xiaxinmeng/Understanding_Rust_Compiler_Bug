 rust
pub fn with_bytes_reader<T>(bytes: &[u8], f: &fn(@Reader) -> T) -> T {
    f(@BytesReader {
        bytes: bytes,
        pos: @mut 0
    } as @Reader)
}

pub fn with_str_reader<T>(s: &str, f: &fn(@Reader) -> T) -> T {
    with_bytes_reader(s.as_bytes(), f)
}
