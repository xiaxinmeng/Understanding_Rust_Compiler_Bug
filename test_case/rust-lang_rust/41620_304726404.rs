rust
let fudge_factor = match magic_value {
            0. ... 0.8 => 9., 
            0. ... 1.0 => 6.,
            0. ... 1.2 => 4.,
            0. ... 2.5 => 3.67, 
            _ => 3.0,
        };
