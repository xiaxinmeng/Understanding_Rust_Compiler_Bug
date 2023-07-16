rust
    for c in s.as_str().chars().rev() {
        match c {
            '\n' | '\r' => n += 1,
            _ => break,
        };
    }
