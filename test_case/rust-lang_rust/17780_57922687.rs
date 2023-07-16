 rust
#![feature(unboxed_closures, overloaded_calls)]

fn main() {
    let mut x = 0u;
    let _f = |&:| x = 42;
}
