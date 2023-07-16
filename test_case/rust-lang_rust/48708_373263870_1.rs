rust
if let Some(x) = x {
    if let Some(y) = y {
        if x == y { /* */ }
    }
}
match (x, y) {
    (Some(x), Some(y)) if x == y => { /* */ }
    _ => {}
}
