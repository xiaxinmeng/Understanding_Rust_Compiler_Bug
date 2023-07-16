 Rust
#[derive(Debug,Copy,Clone)]
struct Foo {
    bar: Bar
}
#[derive(Debug,Copy,Clone)]
struct Bar {
    // lots of data
}
fn doit() -> u32 {
    let foo = Foo { bar: make_bar() };
    debug!("{}", &foo);
    match foo {
        Foo { bar } => frobnicate(&mut bar)
    };
}
