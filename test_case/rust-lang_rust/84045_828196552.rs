rust
    let x = 1.0000001;
    assert!(match x {
        1.00000011 => true,
        _ => false,
    });
