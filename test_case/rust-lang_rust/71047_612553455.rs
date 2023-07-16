rust
#![warn(unreachable_pub)]
pub use inner::{A, P};

mod inner {
    pub struct A;
    pub use double_inner::P;
    mod double_inner {
        pub struct P;
    }
}
