rust
trait Foo {}
impl<'a, T: A> Foo for T
where <T as A>::B<'a>: 'static
{}
