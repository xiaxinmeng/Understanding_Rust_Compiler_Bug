 rust
static A: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn foo() -> usize {
    A.swap(1, Ordering::AcqRel)
}

#[no_mangle]
pub extern "C" fn bar() -> usize {
    A.swap(2, Ordering::AcqRel)
}
