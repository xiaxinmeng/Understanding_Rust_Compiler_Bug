rust
#[derive(PartialEq)]
struct Foo {
    a: i32,
    b: f32,
}

let f = Foo { a: 42, b: 0.0 };
match () {
    () if f == Foo { a: 0, b: 0.0 } => {}
    //~^ ERROR: struct literals are not allowed here
    _ => {}
}
