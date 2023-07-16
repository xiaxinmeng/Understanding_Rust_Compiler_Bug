 rust
// 7025.rs

#[cfg(target_os = "linux")]
fn foo() {}

#[cfg(target_os = "win32")]
fn foo() {}


fn main() {}
