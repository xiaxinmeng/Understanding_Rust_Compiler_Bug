rust
impl<T> _Bar<T> {
    fn _map<U>(x: U) -> _Bar<U> {
        _Bar::<T>(x)
    }
}
