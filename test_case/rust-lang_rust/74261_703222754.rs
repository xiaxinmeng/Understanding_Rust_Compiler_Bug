rust
pub trait Trait<'a> {
    type Output;

    fn output(self) -> Self::Output;
}

impl<'a> Trait<'a> for () {
    type Output = ();

    fn output(self) -> () {
        ()
    }
}

fn confused_hrtb<T, F>(value: T, f: F)
where
    T: for<'a> Trait<'a>,
    F: for<'b> FnOnce(<T as Trait<'b>>::Output),
{
    f(value.output())
}

fn main() -> () {
    confused_hrtb((), |_| ());
}
