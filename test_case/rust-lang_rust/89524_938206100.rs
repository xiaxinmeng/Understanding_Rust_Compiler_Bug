rust
pub fn parse(text: &str) -> toml::Value {
    toml::from_str(text).unwrap()
}
