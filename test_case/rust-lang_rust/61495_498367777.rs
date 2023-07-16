rust
#![feature(const_fn, core_intrinsics)]
pub const fn bar(a: i32, b: i32) -> (i32, bool) {
    unsafe { core::intrinsics::add_with_overflow(a, b) }
}
