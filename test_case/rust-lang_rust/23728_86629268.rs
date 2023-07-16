 rust
impl<T, E> Result<T, E> {
    fn reduce<F, U, X>(self, init: X, action: F) -> U::Value
            where F: Fn(X, T) -> U,
                  U: Async<Error=E, Value=X> {
        unimplemented!();
    }
}
