rust
    unsafe {
    let b = f32x2(0.0, 0.0);

    match (&sin_v2f32(b), &b) {
        (a, b) => {
            if a != b {
                panic!("not equal {:?} {:?}", a, b); // not optimised out unless this panic is replaced with something that does not use `a` or `b`.
            }
        }
    }
    }

