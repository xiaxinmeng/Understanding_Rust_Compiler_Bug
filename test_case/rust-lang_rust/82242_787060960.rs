rust
    for x in (5u32..100u32).step_by(5) {
        for y in 1u32..5u32 {
            let z: u32 = x / y;
            uwriteln!(
                &mut serial,
                "{:?} / {:?} = {:?}\r",
                x.to_le_bytes(),
                y.to_le_bytes(),
                z.to_le_bytes()
            )
            .void_unwrap();
        }
    }
