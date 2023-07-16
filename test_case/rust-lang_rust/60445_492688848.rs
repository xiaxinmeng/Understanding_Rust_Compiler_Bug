rust
let mut x = S {
    a: unsafe { mem::uninitialized() },
    b: foo(),
};
unsafe {
    ffi_init(&mut x.a);
}
