rust
fn x<T: Default>() {
    static a: T = T::default();
}
