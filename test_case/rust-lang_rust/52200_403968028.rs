rust
    let x : libc::c_int = 4;
    let _y : *const libc::c_void = { let t: *const libc::c_int = &x; t}
        as *const libc::c_void;
