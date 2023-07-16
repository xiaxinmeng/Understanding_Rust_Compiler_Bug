rust
#![feature(no_core)]
#![no_core]

mod bar {
    pub struct Baz;
    impl Baz {
        pub fn doit() {}
    }
}

pub use bar::Baz;
