rust
impl<T, U, E> Sum<Result<U, E>> for Result<T, E>
    where T: Sum<U>,
{
    fn sum<I>(iter: I) -> Result<T, E>
        where I: Iterator<Item = Result<U, E>>,
    {
        ResultShunt::process(iter, |iter| iter.sum())
    }
}
