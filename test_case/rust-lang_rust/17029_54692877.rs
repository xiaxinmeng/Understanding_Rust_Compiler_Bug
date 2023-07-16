 rust
fn with<T>(val: T, f: |&mut T|) -> T {
    let mut val = val;
    f(&mut val);
    val
}
