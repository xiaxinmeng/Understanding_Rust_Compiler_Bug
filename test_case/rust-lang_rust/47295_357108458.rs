rust
    match v {
        (1, 2) => (),
        (1, 2) => (), // OK, only a warning
        _ => (),
    }
