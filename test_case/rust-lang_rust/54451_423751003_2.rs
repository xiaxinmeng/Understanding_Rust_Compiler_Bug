 rust
#![crate_type = "lib"]

#[inline(never)]
#[no_mangle]
fn foo() -> u32 {
    unsafe { std::ptr::read_volatile(&42) }
}

pub fn bar() -> u32 {
    foo()
}
