rust
    if a >= 0xffffff80000000000000000000000000_u128 {
        f32::INFINITY
    } else {
        a as f32
    }
