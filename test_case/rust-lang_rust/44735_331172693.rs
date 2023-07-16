rust
#![feature(unboxed_closures)]

fn main() {}
fn foo<F: Fn(*mut &u32)>(_f: F) {}
fn bar<'a>(f: fn(*mut &'a u32)) { foo(f); }
