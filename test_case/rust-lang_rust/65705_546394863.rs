rust
    let ascii_case_difference = b'a' - b'A';
    for idx in 0..len {
        unsafe {
            *ptr.add(idx) -= ascii_case_difference;
        }
    }
