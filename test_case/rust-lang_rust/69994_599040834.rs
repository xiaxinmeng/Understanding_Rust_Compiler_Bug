rust
trait Trait<'a> {}
impl<'a> Trait<'a> for () {}

trait Foo {
    fn foo<'a> ()
    ;
}
impl Foo for () {
    fn foo<'a> ()
    where
        () : Trait<'a>,
    {}
}
