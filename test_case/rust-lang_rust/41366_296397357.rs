
impl<'input, P, I, F, T> Parse<'input>
for Fold<P, I, F>
where
    P: Parse<'input>,
    I: Fn() -> T,
    F: Fn(T, P::Output) -> T,
{
    type Output = T;
}
