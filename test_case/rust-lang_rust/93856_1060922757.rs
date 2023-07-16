rust
struct Foo<T>(T); // `T` is covariant.
fn foo<'b>(x: Foo<for<'a> fn(&'a ())>) {
    let Foo(y): Foo<fn(&'b ())> = x;
}
