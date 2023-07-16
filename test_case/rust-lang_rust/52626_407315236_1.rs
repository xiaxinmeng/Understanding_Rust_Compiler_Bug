rust
    let x = 5;
    let mut x: *const u32 = &x;
    loop {
        let y = 5;
        x = &y;
    }
    0
