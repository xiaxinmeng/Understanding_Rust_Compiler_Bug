rust
#![no_std]
fn foo() {
    unsafe { core::ptr::drop_in_place(&mut 1); }
}
