rust
impl<T, Tab> Insertable<Tab> for Option<T>
where
    T: Insertable<Tab>,
    T::Values: Default,
{
    // ...
}

impl<'a, T, Tab> Insertable<Tab> for &'a Option<T>
where
    Option<&'a T>: Insertable<Tab>,
{
    // ...
}
