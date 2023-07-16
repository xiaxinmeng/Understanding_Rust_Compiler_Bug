rust
    let closure = unsafe { || *p};
    unsafe { unsafe_fn_taking_fn_argument(closure) }
