rust
trait Bar {}
impl Bar for u8 {}

type Foo = impl Bar;

fn foo() -> Foo {
    10
}
