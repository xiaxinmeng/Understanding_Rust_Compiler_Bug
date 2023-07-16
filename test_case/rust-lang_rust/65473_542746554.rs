rust
    let ptr: *const i32;
    let x: usize = ptr as usize;
    let y = x as *const i32;
    assert_eq!(ptr, y);
