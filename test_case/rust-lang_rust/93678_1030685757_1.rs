rs
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn granular_disallow_op_in_unsafe_fn() {
    unsafe {
        #[deny(unsafe_op_in_unsafe_fn)]
        {
            unsf();
        }
    }
}
