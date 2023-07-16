 rust
struct Foo {
    a: i32,
    b: i64
}

impl Foo {
    pub extern fn new(a: i32, b: i64) -> Foo {
        Foo {
            a: a,
            b: b,
        }
    }
}
