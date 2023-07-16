rust
match self {
    Some(x) if predicate(&x) => Some(x),
    _ => None
}
