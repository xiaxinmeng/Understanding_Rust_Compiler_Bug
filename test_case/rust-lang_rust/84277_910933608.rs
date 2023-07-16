rust
#[derive(Debug)]
pub enum SomeError {
    NoStringError,
    NoIntError,
}
struct MyResult(Result<i32, SomeError>);

// allow convert Option<T> i
impl FromResidual<PhantomData<String>> for MyResult {
    fn from_residual(_: PhantomData<String>) -> Self {
        Self(Err(SomeError::NoStringError))
    }
}
impl FromResidual<PhantomData<i32>> for MyResult {
    fn from_residual(_: PhantomData<i32>) -> Self {
        Self(Err(SomeError::NoIntError))
    }
}
