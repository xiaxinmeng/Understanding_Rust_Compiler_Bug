rust
unsafe fn foo() {
    unsafe {
        first_fn();
        second_fn();
    }
    third_fn();
}
