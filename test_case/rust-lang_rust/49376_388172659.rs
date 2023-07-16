rust
struct Bar {}
trait Foo<T = i32> {}
impl Foo for Bar {}

fn foo() -> impl Foo {
    Bar {}
}

fn main() {}
