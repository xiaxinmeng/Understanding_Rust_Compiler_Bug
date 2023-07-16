 rust
enum Foo<'a, T:'a+?Sized> {
    A(T),
    B(&'a T),
}

struct Bar<'a, T:'a+?Sized> {
    b: Foo<'a, T>,
}

fn main() {
    let _ = Bar { b: Foo::A(10) };
    let _ = Bar { b: Foo::B(&"rust") };
}
