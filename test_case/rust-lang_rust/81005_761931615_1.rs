rust
unsafe {
    let mut x: [i32; 3] = [100, 200, 300];
    let p1: *mut i32 = x.as_mut_ptr();
    let p2: *mut i32 = &mut *p1; // intermediate reference

    std::ptr::copy(p1, p2, 1); // miri treated this line as safe
}
