rust
pub trait Func<'a, T> {
    type Output;
    fn invoke(&mut self, x: T) -> Self::Output;
}

impl<'a, T, U, F> Func<'a, T> for &'a mut F
where
    F: FnMut(T) -> U,
{
    type Output = U;
    fn invoke(&mut self, x: T) -> Self::Output {
        self(x)
    }
}
trait Foo {
    fn test<F>(f: F)
    where
        for<'a> &'a mut F: Func<'a, &'a ()>;
}

impl Foo for () {
    fn test<F>(mut f: F)
    where
        for<'a> &'a mut F: Func<'a, &'a ()>,
    {
        (&mut f).invoke(&());
    }
}

fn main() {
    <()>::test(|t| { t }); //error
}
