
impl<'input, P, I, F, T> Parse<'input>
for Fold<P, I, F>
where
    P: Parse<'input>,
    I: Fn() -> T,
    // changed to give the ::Output its own lifetime
    F: for<'x> Fn(T, <P as Parse<'x>>::Output) -> T,
{
    type Output = T;
}
