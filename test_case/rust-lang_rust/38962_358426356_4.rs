rust
enum Foo<'a> {
    // works now too thanks to the explicit owned type
    Bar(Cow<'a, [Foo<'a>], Vec<Foo<'a>>>),

    // can do even this now
    Baz(Cow<'a, Foo<'a>, Box<Foo<'a>>>),
}
