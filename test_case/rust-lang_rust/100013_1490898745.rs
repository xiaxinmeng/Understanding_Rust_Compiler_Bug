rs
trait Foo {
    type Bar<'a: 'b, 'b>;
    fn baz(_f: impl for<'a, 'b> Fn(Self::Bar<'a, 'b>)) {} //ok
}

impl Foo for () {
    type Bar<'a: 'b, 'b> = ();
    fn baz(_f: impl for<'a, 'b> Fn(Self::Bar<'a, 'b>)) {} //fails
}
