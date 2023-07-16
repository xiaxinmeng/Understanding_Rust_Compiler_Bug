
trait Foo {
    fn foo_one(&self) -> &'static str;
    fn foo_two(&self) -> &'static str;
}

default impl<T> Foo for T {
    fn foo_one(&self) -> &'static str {
        "generic"
    }
}

struct MyStruct;
