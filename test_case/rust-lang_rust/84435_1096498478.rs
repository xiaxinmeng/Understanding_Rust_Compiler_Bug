rust
trait Trait<'a> {}
trait Foo<'a>: Trait<'a> + for<'b> Trait<'b> {}
