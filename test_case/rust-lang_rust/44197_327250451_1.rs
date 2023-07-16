rust
    move || while let GeneratorState::Yielded(v) = { let state = g.resume(); state } {
        yield v * x;
    }
