rust
struct Foo<T>(T); // `T` is covariant.

fn foo<'b>(x: Foo<Foo<for<'a> fn(&'a ())>>) {
    let Foo(Foo(y)): Foo<Foo<fn(&'b ())>> = x;
}
