rust
struct Foo<T = impl Copy>(T);

fn x() -> Foo {
    Foo(0)
}
