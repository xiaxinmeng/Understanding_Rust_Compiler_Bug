rust
    match something {
        OtherVariant => { /* ... */ }
        SomeVariant => Box::new({
            let mut count = 0;
            std::iter::from_fn(move || {
                count += 1;

                if count < 6 {
                    Some(count)
                } else {
                    None
                }
            })
        }),
    }

    // vs.

    match something {
        OtherVariant => { /* ... */ }
        SomeVariant => Box::new(std::iter::from_fn(0, |count| {
            *count += 1;

            if *count < 6 {
                Some(*count)
            } else {
                None
            }
        })),
    }
