 rust
fn same_refs<T>(a: &T, b: &T) -> bool {
    let a: *const T = a;
    let b: *const T = b;
    a == b
}
