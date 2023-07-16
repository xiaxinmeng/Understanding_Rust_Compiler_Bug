 rust
fn zomg(foo: &Foo)
    match *foo {
        Inline(ref t, _) => t.zomg(),
        Boxed(ref t) => t.zomg()
}
