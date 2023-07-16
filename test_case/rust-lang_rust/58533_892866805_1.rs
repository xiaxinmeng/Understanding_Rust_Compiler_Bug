rust
fn cpp_example() {
    let mut range = [1, 2, 3];
    let range_ptr = &mut range[..] as *mut [_];
    let mut iter = unsafe { BeginAndEnd::begin(range_ptr) };
    while iter != unsafe { BeginAndEnd::end(range_ptr) } {
        let v = unsafe { *iter.get() };
        println!("{}", v);
        unsafe { iter.move_to_next() };
    }
}

fn rust_equivalent() {
    for v in &mut [1, 2, 3] {
        println!("{}", v);
    }
}
