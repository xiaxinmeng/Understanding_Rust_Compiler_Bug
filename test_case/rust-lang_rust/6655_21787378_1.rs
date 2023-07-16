 rust
struct Foo<'self>(&'self int);

impl<'self> Foo<'self> {
    fn works(&self) -> &'self int { **self }

    fn doesnt<'a>(&'a self) -> &'a int { **self }
}

fn run<'a>(foo: Foo<'a>) -> &'a int {
    foo.doesnt()
}

fn main() {}
