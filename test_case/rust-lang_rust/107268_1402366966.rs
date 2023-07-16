rust
pub trait Trait {
    type Ty<'a>;
}

fn my_fn<T: Trait>(_: T::Ty<'_>) {}

fn test<A, B>() -> impl Fn(A::Ty<'static>)
where
    A: Trait,
    B: for<'a> Trait<Ty<'a> = A::Ty<'a>>,
{
    my_fn::<_> // A and B are both valid options; We pick A!
}
