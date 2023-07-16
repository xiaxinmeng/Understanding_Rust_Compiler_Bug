rust
trait Foo {
    type Bar;
}

trait Baz: Foo {
    const Bar: u32;
    
    fn foo() -> Self::Bar;
}
