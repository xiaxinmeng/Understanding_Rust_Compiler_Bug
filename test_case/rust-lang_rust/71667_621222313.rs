rust
pub use core::intrinsics::transmute;
// -->
#[inline(always)]
pub fn transmute<T, U>(value: T) -> U {
    core::intrinsics::transmute(value)
}
