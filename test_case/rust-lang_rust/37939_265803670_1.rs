rust
impl<U, V> Carrier for Result<U, V> {
    type Success = U;
    type Error = V;

    fn from_success(u: U) -> Result<U, V> {
        Ok(u)
    }

    fn from_error(e: V) -> Result<U, V> {
        Err(e)
    }

    fn translate<T>(self) -> T
        where T: Carrier<Success=U, Error=V>
    {
        match self {
            Ok(u) => T::from_success(u),
            Err(e) => T::from_error(e),
        }
    }
}
