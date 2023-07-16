rust
    match something {
        OtherVariant => { /* ... */ }
        SomeVariant => Box::new(std::iter::successors(Some(1), |count| {
                Some(count + 1).filter(|it| it < 6)
        })),
    }
