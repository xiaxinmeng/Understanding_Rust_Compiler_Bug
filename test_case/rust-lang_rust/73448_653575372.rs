rust
    match 42 {
        // This should be accepted.
        A::<usize>::VALUE => (),
        _ => (),
    }
