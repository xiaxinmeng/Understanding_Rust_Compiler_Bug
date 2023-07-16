rust
match self {
    None => &[],
    Some(x) => std::slice::from_ref(x),
}
