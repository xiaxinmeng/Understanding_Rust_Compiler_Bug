rust
#![feature(const_fn, core_intrinsics)]
pub const fn bar() {
    unsafe { core::intrinsics::assume(true) }
}
