rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

struct S;

impl Fn(u32) -> u32 for S {
    fn call(&self) -> u32 {
        5
    }
}

fn main() {}
