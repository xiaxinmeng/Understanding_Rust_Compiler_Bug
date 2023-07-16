Rust
pub trait Trait<A: ?Sized>
where
    for<'a> A: Trait<&'a Self>,
    for<'a> &'a Self: Trait<Self>,
{
}

impl<U, T> Trait<U> for T
where
    for<'a> T: Trait<&'a U>,
    for<'a> &'a Self: Trait<U>,
{
}
