 rust
fn main() {
    enum Foo {
        Bar { bar: ~str },
        Baz { baz: int }
    }
    match Bar(~"bar") {
        Bar { ref bar } => { println(*bar) } // should print `bar`
        _ => {}
    }

    let mut baz = Baz { baz: 0 };
    match baz {
        Baz { ref mut baz } => { *baz += 1 }
        _ => {}
    }
    printfln!(baz); // the `baz` field should be 1
}
