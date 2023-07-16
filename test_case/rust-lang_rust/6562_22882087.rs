 rust
pub trait <T> MapUtil: Array<T> {
    fn map<S, U: Array<T>>(Self) -> U;
}
