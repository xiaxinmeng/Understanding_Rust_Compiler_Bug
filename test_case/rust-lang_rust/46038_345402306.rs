rust
#![feature(const_fn)]

const fn foo(a: i32) {
    if a > 10 {
        panic!();
    }
}

fn main() {}
