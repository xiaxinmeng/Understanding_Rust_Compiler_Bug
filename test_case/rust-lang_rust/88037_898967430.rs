rust
#![feature(const_trait_impl)]

const fn foo() -> u32 {
    let f = || 42;
    42 //f()
}

fn main() {}
