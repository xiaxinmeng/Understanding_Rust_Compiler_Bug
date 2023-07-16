rust
    match let Some(d) = Some(D(0)) {
        // borrowck doesn't understand that `true` and the pattern matching conincide.
        true => D(1),
        _ => {} // make sure `d` is not available here.
    }
    D(2);
