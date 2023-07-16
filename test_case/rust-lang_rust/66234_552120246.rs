rust
struct Foo {
    a: usize,
    b: bool
}

fn bar(val: Foo) -> Foo {
    let new = Foo {
        a: val.a,
        b: val.b
    };
    new
}
