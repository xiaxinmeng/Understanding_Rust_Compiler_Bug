rust
impl<T, E> ToValue for T
    where
        T: Clone,
        T: for<'a> TryInto<Value<'a>, Error = E>,
        ValueError: From<E>,
{
    fn to_value(&self) -> Result<Value, ValueError> {
        self.clone().try_into().map_err(Into::into)
    }
}
