rust
let mut iter = array
    .into_iter()
    .filter_map(|rslt| {
        try {
            let u8 = u8::try_from(rslt?)?;
            (u8 % 2 == 0).then_some(u8)
        }.transpose()
    });
