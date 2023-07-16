rust
trait Bound<'a> {
    type Assoc;
}

trait Bounded<'a, C: Bound<'a>>
where
    C::Assoc: 'static,
{
}

impl<'a, T, A> Bounded<'a, T> for T
where
    T: Bound<'a, Assoc = A>,
    A: 'static,
{
}
