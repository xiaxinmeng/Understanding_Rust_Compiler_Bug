rust
pub fn foo() -> bool {
    std::env::var("FOO").unwrap_or("0".to_string()) != "BAR"
}
