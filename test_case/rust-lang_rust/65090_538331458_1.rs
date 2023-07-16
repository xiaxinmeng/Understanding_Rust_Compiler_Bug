rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
pub struct Test {
    pub a: u64,
    pub b: u64,
}
pub mod dyn32 {
    pub use super::*;
    use scroll::SizeWith;
    pub struct Test {
        pub a: u32,
        pub b: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Test {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Test {
        #[inline]
        fn clone(&self) -> Test {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    impl ::scroll::ctx::SizeWith<::scroll::Endian> for Test {
        type Units = usize;
        #[inline]
        fn size_with(ctx: &::scroll::Endian) -> Self::Units {
            0 + <u32>::size_with(ctx) + <u32>::size_with(ctx)
        }
    }
}
