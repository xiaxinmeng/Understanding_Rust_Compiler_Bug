rust
match value as i32 {
    x @ 0.. => Some(x), // x has type `i32 is 0..`
    _ => None,
}
