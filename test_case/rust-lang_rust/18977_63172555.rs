 rust
type baz<'r> = |a: &'r str|: 'r;

struct Foo<'a> {
    pub bar: Option<baz<'a>>,
}

fn main() {
    let foo = Foo {
        bar: Some(|_| {}),
    };
}
