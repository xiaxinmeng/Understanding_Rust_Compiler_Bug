 rust
#![crate_type="lib"]

extern {
    fn black_box(ptr: *const u8);
}

pub unsafe fn foo() {
    // Make sure we use the stack
    let x: [u8, ..50] = [0, ..50];
    black_box(x.as_ptr());
}
