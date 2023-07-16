rust
mod sub4 {
    pub const X: usize = 0;
    pub mod inner {
        pub use super::*;
        pub const X: usize = 1;
    }
}

#[doc(inline)]
pub use sub4::inner::*;
