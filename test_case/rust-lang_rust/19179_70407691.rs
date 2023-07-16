 rust
pub mod a {
    use ::b::B;
    use std::*;
    pub struct A {
        i: isize
    }
}
pub mod b {
    use ::a::A;
    use std::*;
    pub struct B {
        i: isize
    }
}
