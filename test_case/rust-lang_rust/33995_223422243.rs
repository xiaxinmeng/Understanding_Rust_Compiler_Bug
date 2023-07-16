 rust
#![allow(dead_code, unused_variables)]

// error: identifier `a` is bound more than once in this parameter list
fn f(a: u8, a: f64) {}

trait Tr {
    // error: identifier `a` is bound more than once in this parameter list
    fn g(a: u8, a: f64) {}

    // No errors
    fn h(a: u8, a: f64);
}

// No errors
type A = fn(a: u8, a: f64);

extern "C" {
    // No errors
    fn k(a: u8, a: f64);
}

fn main() {
}
