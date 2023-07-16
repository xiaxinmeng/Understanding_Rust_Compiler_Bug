rust
pub fn foo(x: Result<u32,!>) -> u32 { x.unwrap_or_else(|e| e) }
