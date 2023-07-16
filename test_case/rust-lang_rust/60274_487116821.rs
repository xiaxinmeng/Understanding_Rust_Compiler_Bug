rust
trait Trait {
    type AssociatedType;
}

impl<'a, T : 'a> Trait for &'a T {
    type AssociatedType = &'a ();
}

/// Calling this with some T when we don't have `T : 'static` leads to an ICE
fn foo<T> (_: &'_ T)
where
    for<'a> &'a T : Trait<AssociatedType = &'a ()>,
{}

/// proof
fn main ()
{
    use ::core::convert::identity as force_local;
    match force_local(()) { ref not_static => {
        foo(&not_static);
    }}
}
