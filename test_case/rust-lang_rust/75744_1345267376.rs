rs
impl<T> ControlFlow<T, T> {
    pub fn value(self) -> T {
        match self {
            ControlFlow::Continue(value) | ControlFlow::Break(value) => value,
        }
    }
}
