Rust
#[repr(C, align(4))]
struct Foo(u8);
#[repr(C, packed(1))]
struct Bar(Foo);
