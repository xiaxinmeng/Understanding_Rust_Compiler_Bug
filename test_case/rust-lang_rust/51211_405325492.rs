rust
#![feature(rust_2018_preview)]
#![warn(rust_2018_compatibility)]

macro_rules! foo {
    () => {
        {
            use myself::f;
            f();
        }
    }
}

mod myself {
    pub fn f() {}
}

fn main() {
    foo!();
    foo!();
}
