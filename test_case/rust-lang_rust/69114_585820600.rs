rust
fn foo(p: *mut &'static u8) {
    let n = 42;
    unsafe {
        *p = &n;
    }
}
