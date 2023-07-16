 rust
pub fn with_str_reader<T>(s: &str, f: &fn(&Reader) -> T) -> T {...}
