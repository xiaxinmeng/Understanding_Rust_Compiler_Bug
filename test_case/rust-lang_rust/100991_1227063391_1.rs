rust
impl<'a, T, Tab, U> Insertable<Tab> for &'a Eq<T, U> // <--- HERE
where
    T: Copy,
    Eq<T, &'a U>: Insertable<Tab>,
{
    type Values = <Eq<T, &'a U> as Insertable<Tab>>::Values;

    fn values(self) -> Self::Values {
        Eq::new(self.left, &self.right).values()
    }
}
