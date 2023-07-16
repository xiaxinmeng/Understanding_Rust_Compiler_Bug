rust
pub trait Try: FromFailure<Self::Failure> {
    type Output;
    /// A type representing a short-circuit produced by this specific `Try` implementation.
    ///
    /// Each `Try` implementation should have its own `Failure` type, so that a custom
    /// conversion can be defined for every combination of the type `?` is used on
    /// (implementing `Try<Failure = F>`), and the return type of the function it is used
    /// within (implementing `FromFailure<F>`).
    ///
    /// (Docs can give an example of using ! if they like)
    type Failure;
    fn from_output(x: Self::Output) -> Self;
    fn branch(self) -> ControlFlow<Self::Failure, Self::Output>;
}

pub trait FromFailure<F = <Self as Try>::Failure> {
    /// Construct Self from a failure type, 
    fn from_failure(failure: F) -> Self;
}
