rust
pub trait Foo: Circular {}

pub trait Circular {
    type Unit;
}

impl<'a> Foo for &'a () {}

impl<'a> Circular for &'a ()
where
    &'a (): Circular<Unit = ()>,
{
    type Unit = ();
}
