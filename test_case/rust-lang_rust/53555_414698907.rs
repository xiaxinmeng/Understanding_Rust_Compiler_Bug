rust
    const FOO : Option<Cell<i32>> = (None, Some(Cell::new(0))).0;
    let _: &'static _ = &FOO; // ERROR: does not live long enough
