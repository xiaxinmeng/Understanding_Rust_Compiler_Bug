rust
pub trait ResolveTo<T> {
    fn resolve(self) -> T;
}
