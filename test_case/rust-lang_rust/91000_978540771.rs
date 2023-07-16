
// Designed to be called from C++ or C.
#[no_mangle]
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn demo(start: *const u16, end: *const u16) {
    for ptr in unsafe { RawIter::new(start, end) } {
        println!("{}", *ptr);
    }
}
