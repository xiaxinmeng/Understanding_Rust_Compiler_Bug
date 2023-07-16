rust
#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
use std::marker::PhantomData;
use std::ops::{ControlFlow, Try, Residual, FromResidual};

mod hacks {
    pub trait TypeThatWorksInHResult {
        fn into_u32(self) -> u32;
        fn from_u32(x: u32) -> Self;
    }
    impl TypeThatWorksInHResult for () {
        fn into_u32(self) -> u32 { 0 }
        fn from_u32(x: u32) { debug_assert!(x == 0); }
    }
    impl TypeThatWorksInHResult for bool {
        // `S_FALSE` is `1`: <https://referencesource.microsoft.com/#windowsbase/Base/MS/Internal/Interop/ErrorCodes.cs,e08462dc3482f421>
        fn into_u32(self) -> u32 { if self { 0 } else { 1 } }
        fn from_u32(x: u32) -> bool { debug_assert!(x <= 1); x == 0 }
    }
    impl TypeThatWorksInHResult for u8 {
        fn into_u32(self) -> u32 { self as _ }
        fn from_u32(x: u32) -> u8 { debug_assert!(x <= 0xFF); x as _ }
    }
    impl TypeThatWorksInHResult for u16 {
        fn into_u32(self) -> u32 { self as _ }
        fn from_u32(x: u32) -> u16 { debug_assert!(x <= 0xFFFF); x as _ }
    }
}

#[repr(transparent)]
pub struct HResult<T: hacks::TypeThatWorksInHResult>(u32, PhantomData<T>);
pub struct HResultResidual(u32); // TODO: use `NegativeI32`, once possible

impl<T: hacks::TypeThatWorksInHResult> Try for HResult<T> {
    type Output = T;
    type Residual = HResultResidual;
    
    fn from_output(x: T) -> Self {
        Self(x.into_u32(), PhantomData)
    }
    fn branch(self) -> ControlFlow<HResultResidual, T> {
        if (self.0 as i32) < 0 {
            ControlFlow::Break(HResultResidual(self.0))
        } else {
            ControlFlow::Continue(T::from_u32(self.0))
        }
    }
}

impl<T: hacks::TypeThatWorksInHResult> FromResidual for HResult<T> {
    fn from_residual(r: HResultResidual) -> Self {
        Self(r.0, PhantomData)
    }
}

impl<T: hacks::TypeThatWorksInHResult> Residual<T> for HResultResidual {
    type TryType = HResult<T>;
}
