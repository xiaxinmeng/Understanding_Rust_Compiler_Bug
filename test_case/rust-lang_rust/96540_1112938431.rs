rust
struct Foo<'a>(&'a ());

fn foo() -> impl Fn(Foo) {
    |_| ()
}

fn main() {}
