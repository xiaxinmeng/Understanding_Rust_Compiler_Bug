rust
pub struct ErrorStack<E> {
    stack: ..,
    inner: E,
}

impl<E> ErrorStack<E> {
    /// Construst new ErrorStack with the caller location on top.
    #[track_caller]
    fn new(e: E) -> Self { ... }

    /// Push location of caller to self.stack
    #[track_caller]
    fn push_caller(&mut self) { ... }

    /// Return a new ErrorStack with the wrapped error converted to F
    fn convert_inner<F: From<E>>(f: F) -> ErrorStack<F> { ... }
}

pub enum MyResult<T, E> {
    Ok(T),
    Err(ErrorStack<E>),
}

pub use MyResult::Ok;
pub use MyResult::Err;

impl<T, E> Try for MyResult<T, E> {
    type Output = T;
    type Residual = MyResult<Infallible, E>;

    /* equivalent to std::result::Result's Try impl */
}

/// Pushes an entry to the stack when one [`MyResult`] is coerced to another using the `?` operator.
impl<T, E, F: From<E>> FromResidual<MyResult<Infallible, E>> for MyResult<T, F> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: MyResult<Infallible, E>) -> Self {
        match residual {
            // seems like this match arm shouldn't be needed, but idk the compiler complained
            Ok(_) => unreachable!(),
            Err(mut e) => {
                e.push_caller();
                Err(e.convert_inner())
            }
        }
    }
}

/// Starts a new stack when a [`std::result::Result`] is coerced to a [`Result`] using `?`.
impl<T, E> FromResidual<std::result::Result<Infallible, E>> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: std::result::Result<Infallible, E>) -> Self {
        match residual {
            // seems like this match arm shouldn't be needed, but idk the compiler complained
            std::result::Result::Ok(_) => unreachable!(),
            std::result::Result::Err(e) => Err(StackError::new(e)),
        }
    }
}
