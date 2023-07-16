 rust
trait Hkt<'a> { type Output; }
trait A where for<'s> <Self::B as Hkt<'s>>::Output: Clone {
    type B: for<'s> Hkt<'s>;
}
trait C: A
    where for<'s> <Self::B as Hkt<'s>>::Output: Clone
{}

impl<'a> Hkt<'a> for usize { type Output = usize; }

struct S;
impl A for S {
    type B = usize;
}
