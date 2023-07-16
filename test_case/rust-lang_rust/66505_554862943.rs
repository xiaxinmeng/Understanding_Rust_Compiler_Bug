rust
trait Foo {}

fn foo<'a, 'b>()
where
    &'a (): Foo,
    &'b (): Foo,
{}
