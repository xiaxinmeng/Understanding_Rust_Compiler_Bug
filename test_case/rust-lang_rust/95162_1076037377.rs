rust
    let Q = val;
    if discriminant(P) == otherwise {
        let ptr = &mut Q as *mut _ as *mut u8;
        unsafe { *ptr = 10; } // Any invalid value for the type
    }

    match (P, Q) {
        (A, A) => {
            // branch1
        }
        _ => {
            // branch2
        }
    }
