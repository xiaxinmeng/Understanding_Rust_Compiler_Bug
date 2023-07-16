rust
enum Foo {
    A(String),
    B
}

Foo::A as i32; // uh oh!
