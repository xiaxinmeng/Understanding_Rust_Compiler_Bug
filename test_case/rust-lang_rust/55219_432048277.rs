rust
pub struct Foo<T>(T);

impl<T> Foo<T> {
    const HASH_LEN: usize = 20;

    fn stuff() {
        let _ = Self::HASH_LEN;
    }
}
