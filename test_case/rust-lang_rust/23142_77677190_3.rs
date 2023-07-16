 rust
    let y = match x {
        Some(val) => &*val,
        None => panic!(),
    };
