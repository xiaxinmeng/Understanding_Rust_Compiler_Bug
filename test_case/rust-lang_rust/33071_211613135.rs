 rust
    let mut var = 0i32;
    let borrow = &mut var;
    move || {
        let borrow_borrow: &&mut i32 = &borrow;
        *borrow = 1;
    };
