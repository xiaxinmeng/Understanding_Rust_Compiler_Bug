rust
trait Trait<'a> {
    type Out;
    fn out(self) -> Self::Out;
}
impl<'a> Trait<'a> for u8 {
    type Out = u8;
    fn out(self) -> Self::Out {
        self
    }
}
fn call<F, D>(d: D, f: F) where
    for<'a> D: Trait<'a>,
    for<'a> F: Fn(<D as Trait<'a>>::Out),
{
    f(d.out())
}
fn main() {
    call(5, |_| ());
}
