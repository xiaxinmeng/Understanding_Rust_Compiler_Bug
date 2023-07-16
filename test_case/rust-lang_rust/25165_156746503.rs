 rust
struct Wrapper<F>(F);
struct Foo;

impl Foo {
    fn stuff(self) { }
}

fn api<F: Fn(Foo)>(f: Wrapper<F>) { }

fn main() {
    api(Wrapper(|b| {
        b.stuff(); //~ ERROR the type of this value must be known in this context
    }))
}
