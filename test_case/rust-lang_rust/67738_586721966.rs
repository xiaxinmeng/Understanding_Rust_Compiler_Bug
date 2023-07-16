rust
impl<'a, D, E> Deserializer<'a> for Wrapper<D>
where
    D: Deserializer<'a, Error = E>,
    E: std::error::Error + 'static,
{
    type Error = MyError<E>;
    
    fn deserialize_string(self) -> Result<(), Self::Error> {
        self.0.deserialize_string().map_err(MyError::Deser)
    }
}
