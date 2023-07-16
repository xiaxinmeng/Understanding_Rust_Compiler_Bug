 rust
struct Foo {
    a: i32,
    b: i32
}

impl Foo {
    pub extern fn new(a: i32, b: i32) -> Foo {
        Foo {
            a: a,
            b: b,
        }
    }
}
