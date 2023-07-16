rust
trait Trait {
    type Ty;
}

// erroneous `Ty` impl
impl Trait for () {
    // missing `Ty` impl
}

// `'lt` is not constrained by the erroneous `Ty`
impl<'lt, T> Trait for Box<T>
where
    T: Trait<Ty = &'lt ()>,
{
    type Ty = &'lt ();
}

// unconstrained lifetime appears in implied bounds
fn test(_: <Box<()> as Trait>::Ty) {}
