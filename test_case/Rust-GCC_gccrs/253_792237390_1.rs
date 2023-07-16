Rust
'label: loop {
    match drop_temps { $cond } {
        true => $body,
        _ => break,
    }
}
