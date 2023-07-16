 rust
fn f() {}
type T = &'static extern "Rust" fn();
pub static C: T = &f;
