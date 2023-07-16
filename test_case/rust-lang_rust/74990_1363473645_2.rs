rust
#[link(...)]
extern "C-unwind" {
    // A C++ function that may throw an exception
    fn may_throw();
}

#[no_mangle]
extern "C-unwind" fn rust_passthrough() {
    let b = Box::new(5);
    unsafe { may_throw(); }
    println!("{:?}", &b);
}
