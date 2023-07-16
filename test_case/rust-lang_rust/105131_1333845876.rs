
pub trait Gen<'source> {
    type Output;

    fn gen(&self) -> Self::Output
    where
        Self: for<'s> Gen<'s, Output = Self::Output>;
}
