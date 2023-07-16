rust
match Some(8) {
    Some(1) => 1,
    Some(2) => 2,
    Some(n) if n % 2 == 0 => 3,
    _ => 4,
}
