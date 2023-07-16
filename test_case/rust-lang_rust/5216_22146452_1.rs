 rust
 fn f() {}
struct S(&'static extern "Rust" fn());
pub static C: S = S(&f);
