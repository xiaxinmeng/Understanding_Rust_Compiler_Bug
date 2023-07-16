rust
    let a = Cell::new(0);
    {
        let mut x = a.copy_update();
        *a.copy_update() += 1;
        *x += 1;
    }
    assert_eq!(a.get(), 1); // not 2
