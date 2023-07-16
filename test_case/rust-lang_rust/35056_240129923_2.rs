 rust
impl<T,U,E,F> Carrier<Result<U,F>> for Result<T,E>
    where E: Into<F>
{
    type Ok = T;
    fn is_ok(&self) -> bool { self.is_ok() }
    fn unwrap_into_ok(self) -> Self::Ok { self.unwrap() }
    fn unwrap_into_error(self) -> { Err(F::from(self.unwrap_err())) }
}
