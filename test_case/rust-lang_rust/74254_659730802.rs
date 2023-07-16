rust
fn make<T, F: Fn() -> T, const N: usize>(constructor: F) -> [T; N];
