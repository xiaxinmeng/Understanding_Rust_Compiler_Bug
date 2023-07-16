rust
    match (1) { // Warns, as expected
        (_) => {} // Doesn't warn, but should for consistency
    }
