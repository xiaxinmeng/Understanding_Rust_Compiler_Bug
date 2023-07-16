rust
impl<E, F: From<E>> From<ErrorStack<E>> for ErrorStack<F> {
    #[track_caller]
    fn from(mut e: ErrorStack<E>) -> Self {
        e.push_caller();
        e.convert_inner()
    }
}
