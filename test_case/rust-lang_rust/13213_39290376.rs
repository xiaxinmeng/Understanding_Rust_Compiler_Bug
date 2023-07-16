 Rust
#![crate_type = "lib"]

pub use private::P;

pub struct S {
    p: P,
}

mod private { // `pub mod` solves problem
    pub struct P {
        p: i32,
    }
    pub static THREE: P = P { p: 3 };
}

pub static A: S = S { p: private::THREE };
