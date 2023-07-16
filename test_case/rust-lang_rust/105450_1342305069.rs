
impl<T> TryFrom<Foo> for Option<T>
where
    T: TryFrom<Foo, Error = Infallible>,
{
    type Error = Infallible;

    fn try_from(value: Foo) -> Result<Self, Self::Error> {
        Ok(Some(value.x))
    }
}
