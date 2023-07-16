 rust
fn foo() {}
fn bar() {}
fn baz() {}

pub static FUNCS: &'static [extern "Rust" fn()] = &[foo, bar, baz];
