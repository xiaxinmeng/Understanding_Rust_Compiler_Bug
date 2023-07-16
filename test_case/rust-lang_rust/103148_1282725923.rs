rust
impl<T, E, U> const From<T> for Result<U, E>
where
    U: ~const TryFrom<T, Error = E>,
{
    fn from(t: T) -> Self {
        U::try_from(t)
    }
}
