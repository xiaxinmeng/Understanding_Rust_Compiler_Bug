rust
    let mut a = 1;
    let b = &mut a;
    {
        let foo = |i| {};
        foo(&mut *b);
    }
    {
        let foo = |i| {};
        foo(&mut *b);
    }
