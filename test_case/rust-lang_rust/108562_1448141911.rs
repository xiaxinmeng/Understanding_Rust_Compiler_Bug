rust
pub struct Bar<S>(S);

pub trait Foo {}

impl<S> Foo for Bar<S>
where
    for<'a> &'a S: IntoIterator,
    for<'a> <&'a S as IntoIterator>::Item: Foo,
{
}
