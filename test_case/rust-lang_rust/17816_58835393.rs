 rust
#![feature(overloaded_calls, unboxed_closures)]

fn main() {
    let f = |:| -> &str "";
    f();
}
