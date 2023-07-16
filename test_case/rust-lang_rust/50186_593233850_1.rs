rust
    let a = Cell::new(0);
    a.update(|x| {
        a.update(|x| *x += 1);
        *x += 1;
    });
    assert_eq!(a.get(), 1); // not 2
