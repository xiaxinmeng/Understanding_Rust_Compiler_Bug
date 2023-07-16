rust
struct MyOption<T>(Result<T, PhantomData<T>>);

// let any Option<T> be MyOption<T>
impl<T> From<Option<T>> for MyOption<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(val) => Self(Ok(val)),
            None => Self(Err(PhantomData)),
        }
    }
}

// Allow '?' operator for MyOption
impl<T> FromResidual<PhantomData<T>> for MyOption<T> {
    fn from_residual(o: PhantomData<T>) -> Self {
        Self(Err(o))
    }
}
impl<T> Try for MyOption<T> {
    type Output = T;
    type Residual = PhantomData<T>;
    fn from_output(output: Self::Output) -> Self {
        MyOption(Ok(output))
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(val) => ControlFlow::Continue(val),
            Err(err) => ControlFlow::Break(err),
        }
    }
}
