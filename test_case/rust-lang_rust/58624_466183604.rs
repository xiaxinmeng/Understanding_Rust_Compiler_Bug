rust
trait Foo {}

fn foo() -> impl Foo {
    loop {}
}
