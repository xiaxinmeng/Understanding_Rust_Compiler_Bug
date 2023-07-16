rust
struct Foo;
impl Foo {
    fn method<T>(self)
        // bound that never holds, `Vec<T>` instead of `T` to bypass `_: Trait`.
        where
            Vec<T>: Copy // this line doesn't show up in lacks_span error output
    {}
}

fn has_span() { Foo::method(Foo) }

fn lacks_span() { Foo.method() }
