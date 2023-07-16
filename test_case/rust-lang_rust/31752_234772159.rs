 rust
    let mut tmp = 1;
    let c = &mut tmp;

    vec![0;10].iter().flat_map(|&a| {
        (0..10).map(move |b|{
            a + b + *c
        })
    });
