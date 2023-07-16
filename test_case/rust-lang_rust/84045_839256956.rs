rust
    let x: f32 = 1.0000001;
    assert!(match x {
        1.00000011..=2.0 => true,
        _ => false,
    });
