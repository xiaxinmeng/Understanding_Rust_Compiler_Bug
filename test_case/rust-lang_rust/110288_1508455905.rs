rust
fn zero_sized<T>() -> &'static [T; 0] {
    &[]
}
