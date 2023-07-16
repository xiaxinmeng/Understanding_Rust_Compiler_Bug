rust
    for x in (5u16..100u16).step_by(5) {
        for y in 1u16..5u16 {
            let z: u32 = x as u32 / y as u32;
            uwriteln!(&mut serial, "{} / {} = {:?}\r", x, y, z.to_le_bytes()).void_unwrap();
        }
    }
